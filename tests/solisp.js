const anchor = require('@project-serum/anchor');

describe('solisp', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Solisp;
    const tx = await program.rpc.run('(transfer 0 1 2 4)',
        {
            accounts: {
                tokenProgram: new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
                user: provider.wallet.publicKey,
            },
            remainingAccounts: [
                { pubkey: new anchor.web3.PublicKey('Ha2s5qVfW8HFpTwkEfuJm6NLJSSoBgLSKTTkMaCoVKpS'), isWritable: true, isSigner: false },
                { pubkey: new anchor.web3.PublicKey('AC5aWi4GXaCMfXPKTfASPCEuvNpQs3KgccTYqywqR8HM'), isWritable: true, isSigner: false },
            ],
        }
    );
    console.log("Your transaction signature", tx);
  });
});
