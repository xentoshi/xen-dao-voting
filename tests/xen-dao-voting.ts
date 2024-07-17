import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { XenDaoVoting } from "../target/types/xen_dao_voting";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("xen-dao-voting", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.XenDaoVoting as Program<XenDaoVoting>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  console.log("Wallet Public Key:", provider.wallet.publicKey.toBase58());

  let daoPda: PublicKey;
  let proposalPda: PublicKey;

  before(async () => {
    [daoPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("dao")],
      program.programId
    );
  });

  it("Initializes the DAO", async () => {
    try {
      await program.methods
        .initialize("Test DAO")
        .accounts({
          dao: daoPda,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const daoAccount = await program.account.dao.fetch(daoPda);
      expect(daoAccount.name).to.equal("Test DAO");
      expect(daoAccount.authority.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    } catch (error) {
      console.error("Error initializing DAO:", error);
      throw error;
    }
  });

  it("Creates a proposal", async () => {
    try {
      const daoAccount = await program.account.dao.fetch(daoPda);
      [proposalPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), daoPda.toBuffer(), Buffer.from([daoAccount.proposalCount])],
        program.programId
      );

      await program.methods
        .createProposal("Test Proposal")
        .accounts({
          dao: daoPda,
          proposal: proposalPda,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const proposalAccount = await program.account.proposal.fetch(proposalPda);
      expect(proposalAccount.description).to.equal("Test Proposal");
      expect(proposalAccount.isActive).to.be.true;
    } catch (error) {
      console.error("Error creating proposal:", error);
      throw error;
    }
  });

  it("Votes on a proposal", async () => {
    try {
      await program.methods
        .vote(true)
        .accounts({
          dao: daoPda,
          proposal: proposalPda,
          user: provider.wallet.publicKey,
        })
        .rpc();

      const proposalAccount = await program.account.proposal.fetch(proposalPda);
      expect(proposalAccount.yesVotes.toNumber()).to.equal(1);
      expect(proposalAccount.noVotes.toNumber()).to.equal(0);
    } catch (error) {
      console.error("Error voting on proposal:", error);
      throw error;
    }
  });

  it("Fails to vote twice on the same proposal", async () => {
    try {
      await program.methods
        .vote(false)
        .accounts({
          dao: daoPda,
          proposal: proposalPda,
          user: provider.wallet.publicKey,
        })
        .rpc();

      expect.fail("The transaction should have failed");
    } catch (error) {
      if (error instanceof anchor.AnchorError) {
        expect(error.error.errorCode.code).to.equal("AlreadyVoted");
      } else {
        throw error;
      }
    }
  });

  it("Closes a proposal", async () => {
    try {
      await program.methods
        .closeProposal()
        .accounts({
          dao: daoPda,
          proposal: proposalPda,
          user: provider.wallet.publicKey,
        })
        .rpc();

      const proposalAccount = await program.account.proposal.fetch(proposalPda);
      expect(proposalAccount.isActive).to.be.false;
    } catch (error) {
      console.error("Error closing proposal:", error);
      throw error;
    }
  });

  it("Fails to vote on a closed proposal", async () => {
    try {
      await program.methods
        .vote(false)
        .accounts({
          dao: daoPda,
          proposal: proposalPda,
          user: provider.wallet.publicKey,
        })
        .rpc();

      expect.fail("The transaction should have failed");
    } catch (error) {
      if (error instanceof anchor.AnchorError) {
        expect(error.error.errorCode.code).to.equal("ProposalNotActive");
      } else {
        throw error;
      }
    }
  });
});