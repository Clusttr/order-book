import {
    createSignerFromKeypair,
    keypairIdentity,
    generateSigner,
    PublicKey,
    percentAmount
} from "@metaplex-foundation/umi";
import { createFungibleAsset, TokenStandard } from "@metaplex-foundation/mpl-token-metadata"
import {createUmi} from "@metaplex-foundation/umi-bundle-defaults";
import {web3JsEddsa} from "@metaplex-foundation/umi-eddsa-web3js";
import {web3JsRpc} from "@metaplex-foundation/umi-rpc-web3js";
import {fetchHttp} from "@metaplex-foundation/umi-http-fetch";
import {mplCandyMachine} from "@metaplex-foundation/mpl-candy-machine";
import bs58 from "bs58";

const rpc = 'https://api.devnet.solana.com';
const umi = createUmi(rpc)
    .use(web3JsEddsa())
    .use(web3JsRpc(rpc))
    .use(fetchHttp())
    .use(mplCandyMachine())

const assetURI = "http://localhost:8899"

export async function createToken(secret: Uint8Array) {
    //setup
    const mKeypair = umi.eddsa.createKeypairFromSecretKey(secret)
    const signer = createSignerFromKeypair(umi, mKeypair)
    umi.use(keypairIdentity(signer))

    //create asset
    const mint = generateSigner(umi)
    const transaction = await createFungibleAsset(
        umi,
        {
            mint,
            name: "Clusttr Real World Asset",
            symbol: "cRWA",
            uri: assetURI,
            isMutable: true,
            sellerFeeBasisPoints: percentAmount(1.5),
            creators: [
                { address: umi.payer.publicKey, verified: true, share: 100},
                { address: signer.publicKey, verified: true, share: 0}
            ],
        }
    ).sendAndConfirm(umi)
    const txSig = bs58.encode(transaction.signature)
    return {token: mint.publicKey, txSig}
}

function mintToken(amount: number) {

}