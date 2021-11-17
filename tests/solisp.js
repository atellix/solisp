const anchor = require('@project-serum/anchor');

describe('solisp', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Solisp;
    const tx = await program.rpc.run('(begin (let ((foo 12) (bar (+ 4 3)) (blah "stuff")) (print foo) (print bar) (print blah) (* foo bar)))');
/*    const tx = await program.rpc.run('(let ((bal (cmd "token_balance" 0))) (/ (* bal 5) 1000))',
        {
            remainingAccounts: [
                { pubkey: new anchor.web3.PublicKey('Ha2s5qVfW8HFpTwkEfuJm6NLJSSoBgLSKTTkMaCoVKpS'), isWritable: false, isSigner: false },
                { pubkey: new anchor.web3.PublicKey('AC5aWi4GXaCMfXPKTfASPCEuvNpQs3KgccTYqywqR8HM'), isWritable: false, isSigner: false },
            ],
        }
    );*/
/*    const tx = await program.rpc.run('(cmd "token_transfer" 0 1 2 3 (+ 7 4) 4)',
        {
            remainingAccounts: [
                { pubkey: new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'), isWritable: false, isSigner: false },
                { pubkey: new anchor.web3.PublicKey('Ha2s5qVfW8HFpTwkEfuJm6NLJSSoBgLSKTTkMaCoVKpS'), isWritable: true, isSigner: false },
                { pubkey: new anchor.web3.PublicKey('AC5aWi4GXaCMfXPKTfASPCEuvNpQs3KgccTYqywqR8HM'), isWritable: true, isSigner: false },
                { pubkey: provider.wallet.publicKey, isWritable: false, isSigner: true },
            ],
        }
    );*/
    console.log("Your transaction signature", tx);
  });
});
