import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor";
import { OrderBook } from "../target/types/order_book"
import { createToken } from "../app/utils/token";

describe("direct sales", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider)

    const payer = provider.wallet as anchor.Wallet
    const program = anchor.workspace.OrderBook as Program<OrderBook>

    it("list_asset", async () => {
        const result = await  createToken(payer.payer.secretKey)
        console.log({result})
    })
})