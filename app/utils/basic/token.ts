import * as fs from "fs";
import * as web3 from "@solana/web3.js";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import {createMint} from "@solana/spl-token";


const endpoint = "http://localhost:8899"
const connection = new web3.Connection(endpoint)

function loadWalletKey(): web3.Keypair {
    return web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync("/Users/matthewchukwuemeka/.config/solana/id.json").toString()))
    )
}

const payer = Keypair.fromSecretKey(loadWalletKey().secretKey)

export async function requestAirdrop() {
    const airdropSignature = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL)
    await connection.confirmTransaction(airdropSignature)
    return airdropSignature.toString()
}

export async function createMyMint() {
    console.log("####creating mint####")
    const mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        payer.publicKey,
        0
    )
    return mint.toBase58()
}