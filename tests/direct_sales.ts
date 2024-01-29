import * as anchor from "@coral-xyz/anchor"
import {Program, Provider} from "@coral-xyz/anchor";
import {DirectSales} from "../target/types/direct_sales"
import {createToken} from "../app/utils/metaplex/token";
import {createMyMint} from "../app/utils/basic/token"
import {Connection, Keypair, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
// import {publicKey, publicKeyBytes} from "@metaplex-foundation/umi";
import {getOrCreateAssociatedTokenAccount} from "@solana/spl-token";
import {assert} from "chai";
import {publicKey} from "@metaplex-foundation/umi";

describe("direct sales", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider)

    const mintKeypair = Keypair.fromSecretKey(new Uint8Array([
        18, 228, 169, 213, 37, 58, 118, 81, 46, 235, 191,
        96, 163, 121, 252, 193, 83, 219, 141, 51, 127, 150,
        22, 76, 61, 51, 177, 211, 6, 113, 30, 60, 76,
        170, 98, 238, 151, 134, 10, 79, 44, 235, 21, 153,
        78, 214, 29, 98, 232, 60, 247, 239, 141, 6, 106,
        45, 252, 150, 135, 34, 208, 154, 247, 145
    ]))

    const payer = provider.wallet as anchor.Wallet
    const program = anchor.workspace.DirectSales as Program<DirectSales>

    const [INVENTORY_PDA] = PublicKey.findProgramAddressSync([
        Buffer.from("inventory"), mintKeypair.publicKey.toBuffer(), //publicKeyBytes(NFTMint)
    ], program.programId)
    const [TOKEN_VAULT_PDA] = PublicKey.findProgramAddressSync([
        Buffer.from("vault"), mintKeypair.publicKey.toBuffer() //publicKeyBytes(NFTMint)
    ], program.programId)

    async function mintATA() {
        return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer.payer,
            mintKeypair.publicKey,
            payer.publicKey
        )
    }

    const usdcPubKey = new PublicKey("HKagbtJvkDd9n5pSumWq7HsQTJLF7skZsrdt8iyqjv5D")
    async function usdcATA() {
        return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer.payer,
            usdcPubKey,
            payer.publicKey
        )
    }

    it("add asset", async () => {
        const initInventory = await program.account.inventory.fetch(INVENTORY_PDA)

        const amount = new anchor.BN(1)
        const pricePerToken = new anchor.BN(102)
        const userATA = await mintATA()
        const tx = await program.methods.add(amount, pricePerToken)
            .accounts({
                signer: payer.publicKey,
                signerTokenAccount: userATA.address,
                inventory: INVENTORY_PDA,
                tokenVault: TOKEN_VAULT_PDA,
                mint: mintKeypair.publicKey,
            }).rpc()

        //confirm transaction
        const latestBlockhash = await program.provider.connection.getLatestBlockhash()
        await program.provider.connection.confirmTransaction({
            signature: tx,
            blockhash: latestBlockhash.blockhash,
            lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
        })

        //get vault details
        const inventory = await program.account.inventory.fetch(INVENTORY_PDA)
        assert(inventory.price.eq(pricePerToken), `Correct price set: $${inventory.price.toNumber()}`)
        assert(inventory.amount.toNumber() >= initInventory.amount.toNumber() + amount.toNumber(),
            `Token Added: ${inventory.amount.toNumber()}`)

        print("add asset", () => {
            console.log(`Price set to: $${inventory.price.toNumber()}`)
            console.log(`Token Added: ${amount.toNumber()} | Initial Tokens : ${initInventory.amount.toNumber()} | Total Tokens : ${inventory.amount.toNumber()}`)
        })
    })

    it("update price", async () =>  {
        const initInventory = await program.account.inventory.fetch(INVENTORY_PDA)

        const newPrice = new anchor.BN(47)
        const tx = await program.methods.updateAssetPrice(newPrice)
            .accounts({
                signer: payer.publicKey,
                inventory: INVENTORY_PDA,
                mint: mintKeypair.publicKey
            }).rpc()

        await confirmTx(tx, program.provider)

        const inventory = await program.account.inventory.fetch(INVENTORY_PDA)
        assert(inventory.price.eq(newPrice), `Failed to set new price: ${newPrice.toNumber()}`)

        print("update prince", () => {
            console.log(`Previous Price: ${initInventory.price.toNumber()} | New Price: ${inventory.price.toNumber()}`)
        })
    })

    it("withdraw token", async () => {
        const initInventory = await program.account.inventory.fetch(INVENTORY_PDA)

        const amount = new anchor.BN(1)
        const signerATA = await mintATA()
        const tx = await program.methods.withdraw(amount)
            .accounts({
                signer: payer.publicKey,
                tokenVault: TOKEN_VAULT_PDA,
                inventory: INVENTORY_PDA,
                signerTokenAccount: signerATA.address,
                mint: mintKeypair.publicKey
            }).rpc()

        await confirmTx(tx, program.provider)

        const inventory = await program.account.inventory.fetch(INVENTORY_PDA)
        assert(inventory.amount.toNumber() == initInventory.amount.toNumber() - amount.toNumber(),
            `Failed to withdraw: ${amount.toNumber()}`)
    })

    it.only("buy asset", async () => {
        const initInventory = await program.account.inventory.fetch(INVENTORY_PDA)

        const signerUSDCAcc = await usdcATA()
        const signerMintAcc = await mintATA()

        const amount = new anchor.BN(1)
        const tx = await  program.methods.buy(amount)
            .accounts({
                signer: payer.publicKey,
                signerUsdcAccount: signerUSDCAcc.address, // f change to another payer
                signerMintAccount: signerMintAcc.address, // f change to another payer
                creatorAccount: payer.publicKey, // g
                creatorsUsdcAccount: signerUSDCAcc.address, // g
                usdcMint: usdcPubKey, // g
                // priceList: payer.publicKey, //r
                tokenVault: TOKEN_VAULT_PDA, //g
                inventory: INVENTORY_PDA, //g
                mint: mintKeypair.publicKey //g
            })
            .signers([])
            .rpc()

        console.log({tx})
    })
})

function print(title: string, action: () => void) {
    console.log(`###${title}: START###`)
    action()
    console.log(`###${title}: END###`)
}

async function confirmTx(tx: string, provider: Provider) {
    const latestBlockhash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
    })
}