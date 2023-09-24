import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { SolanaFormsDemo } from "../target/types/solana_forms_demo";

describe("solana-forms-demo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaFormsDemo as Program<SolanaFormsDemo>;

  it("Is initialized!", async () => {
    // Human's wallet
    const human = Keypair.generate();
    const connection = anchor.AnchorProvider.env().connection;
    // Request sol airdrop (for human to be able to do transactions)
    await requestAirdrop(connection, human.publicKey, LAMPORTS_PER_SOL)

    // Add your test here.
    //const tx = await program.methods.create().rpc();

    //     console.log("Your transaction signature", tx);
    // Generate a seed for the main form account
    const name = "MainForm";
    const nameBytes = Buffer.from(name);

    // Find the bump seed for the main form account
    const [mainFormAddress, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [nameBytes, human.publicKey.toBuffer()],
        program.programId
    );
    // Define the parameters for the create function.
    const fr5pqi = new anchor.BN(1_234);
//     const frduif = ["hello", "world"];
//     const fr6i34 = ["foo", "bar"];
    const fr8xjs = "baz";

    // Create the main form account
    const tx = await program.methods.create(
            fr5pqi,
//             frduif,
//             fr6i34,
//            fr8xjs
        ).accounts(
            {
                mainForm: mainFormAddress,
                authority: human.publicKey,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
            }
        ).signers(
            [human]
        ).rpc();
//     const tx = await program.rpc.create(
//         fr5pqi,
//         frduif,
//         fr6i34,
//         fr8xjs,
//         {
//             accounts: {
//                 mainForm: mainFormAddress,
//                 authority: human.publicKey,
//                 rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//                 systemProgram: anchor.web3.SystemProgram.programId,
//             },
//             signers: [human],
//         }
//     );
    console.log("Your transaction signature", tx);

    // Fetch the main form account and check its state
    const mainForm = await program.account.mainForm.fetch(mainFormAddress);
    console.log("Your main form account", mainForm);
//     assert.ok(mainForm.signer_address.equals(human.publicKey));
//     assert.equal(mainForm.version, 0);
//     assert.equal(mainForm.fr_5pqi, 0);
//     assert.equal(mainForm.fr_duif.length, 0);
//     assert.equal(mainForm.fr_8xjs.length, 0)

  });
});

async function requestAirdrop(connection: Connection, address: anchor.web3.PublicKey, lamports: number) {
    const tx = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(tx);
}