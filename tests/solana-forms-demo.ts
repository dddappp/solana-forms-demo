import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hello } from "../target/types/hello";
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("hello", () => {
  let provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const authority = provider.wallet.publicKey;

  const program = anchor.workspace.SolanaFormsDemo as Program<SolanaFormsDemo>;


  it("Is created!", async () => {

    // ----------------------------------------------------------
    // Human's wallet
    const human = Keypair.generate();
    const connection = anchor.AnchorProvider.env().connection;
    // Request sol airdrop (for human to be able to do transactions)
    await requestAirdrop(connection, human.publicKey, LAMPORTS_PER_SOL)
    // ----------------------------------------------------------

    let [mainForm] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("MainForm"),
            human.publicKey.toBuffer()
        ],
        program.programId
    );

    // Add your test here.
    const tx = await program.methods.create(
        new anchor.BN(1_234),
        ["A", "B"],
        ["A", "B"],
        "hello",
        "1",
        new anchor.BN(2),
        "",
        new anchor.BN(3),
        ["2023-10-10", "2023-10-11"],
        ["2023-10-10", "2023-10-11"],
        new anchor.BN(8),
        ["10:10:00", "10:11:00"],
        ["A"],
        ["1", "3"],
        "2023-10-10",
        "y@test.org"
    ).accounts(
        {
           mainForm,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx);

    //
    // update
    const tx_2 = await program.methods.update(
        new anchor.BN(1_234),
        ["B"],
        ["A", "B"],
        "hello",
        "1",
        new anchor.BN(2),
        "",
        new anchor.BN(3),
        ["2023-10-10", "2023-10-11"],
        ["2023-10-10", "2023-10-11"],
        new anchor.BN(8),
        ["10:10:00", "10:11:00"],
        ["A"],
        ["1", "3"],
        "2023-10-10",
        "z@test.org"
    ).accounts(
        {
           mainForm,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx_2);
    //

    // Fetch the state struct from the network.
    const accountState = await program.account.mainForm.fetch(mainForm);
    console.log("account state: ", accountState);

  });
});

async function requestAirdrop(connection: Connection, address: anchor.web3.PublicKey, lamports: number) {
    const tx = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(tx);
}