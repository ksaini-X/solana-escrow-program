use {
    anchor_lang::{
        Discriminator, prelude::SolanaSysvar, solana_program::{clock::Clock, rent::Rent, system_instruction}
    }, anchor_spl::{
        associated_token::get_associated_token_address,
        token::{
            self, Mint, spl_token::instruction::{initialize_mint, mint_to_checked}
        },
    }, litesvm::LiteSVM, solana_keypair::Keypair, solana_message::{AccountMeta, Instruction}, solana_signer::Signer, solana_transaction::Transaction
};

#[test]
fn test_make() {
    let program_id = escrow::id();
    let bytes = include_bytes!("../../../target/deploy/escrow.so");

    let mut svm = LiteSVM::new();
    svm.add_program(program_id, bytes).unwrap();

    let maker = Keypair::new();
    let taker = Keypair::new();

    svm.airdrop(&maker.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&taker.pubkey(), 1_000_000_000).unwrap();

    let mint_a = Keypair::new();
    let mint_b = Keypair::new();

    let rent = Rent::default().minimum_balance(Mint::LEN);

    let mint_acc_a = system_instruction::create_account(
        &maker.pubkey(),
        &mint_a.pubkey(),
        rent,
        Mint::LEN as u64,
        &token::ID,
    );
    let mint_acc_b = system_instruction::create_account(
        &maker.pubkey(),
        &mint_b.pubkey(),
        rent,
        Mint::LEN as u64,
        &token::ID,
    );

    let init_mint_a =
        initialize_mint(&token::ID, &mint_a.pubkey(), &maker.pubkey(), None, 6).unwrap();
    let init_mint_b =
        initialize_mint(&token::ID, &mint_b.pubkey(), &maker.pubkey(), None, 6).unwrap();

    let token_a_trxn = Transaction::new_signed_with_payer(
        &[mint_acc_a, init_mint_a],
        Some(&maker.pubkey()),
        &[&maker, &mint_a],
        svm.latest_blockhash(),
    );
    let token_b_trxn = Transaction::new_signed_with_payer(
        &[mint_acc_b, init_mint_b],
        Some(&maker.pubkey()),
        &[&maker, &mint_b],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(token_a_trxn);
    let _ = svm.send_transaction(token_b_trxn);

    let assc_token_a_maker = get_associated_token_address(&maker.pubkey(), &mint_a.pubkey());
    let _assc_token_b_maker = get_associated_token_address(&maker.pubkey(), &mint_b.pubkey());

    let _assc_token_a_taker = get_associated_token_address(&maker.pubkey(), &mint_a.pubkey());
    let assc_token_b_taker = get_associated_token_address(&maker.pubkey(), &mint_b.pubkey());

    let ix1 = mint_to_checked(
        &token::ID,
        &mint_a.pubkey(),
        &assc_token_a_maker,
        &maker.pubkey(),
        &[&maker.pubkey()],
        100,
        6,
    )
    .unwrap();
    let ix2 = mint_to_checked(
        &token::ID,
        &mint_a.pubkey(),
        &assc_token_b_taker,
        &maker.pubkey(),
        &[&maker.pubkey()],
        100,
        6,
    )
    .unwrap();

    let token_mint_trxn = Transaction::new_signed_with_payer(
        &[ix1, ix2],
        Some(&maker.pubkey()),
        &[&maker],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(token_mint_trxn);

    let make_disc = escrow::instruction::Make::DISCRIMINATOR;
    let mut make_ins_data: Vec<u8> = Vec::new();
    make_ins_data.extend_from_slice(&make_disc);
    make_ins_data.extend_from_slice(&100u64.to_be_bytes());
    make_ins_data.extend_from_slice(&100u64.to_be_bytes());
    make_ins_data.extend_from_slice(&(10_000i64).to_le_bytes());

    let (escrow, _) = anchor_lang::prelude::Pubkey::find_program_address(&[
        b"escrow".as_ref(), 
        maker.pubkey().as_ref(),
        taker.pubkey().as_ref(),
        mint_a.pubkey().as_ref(),
        mint_b.pubkey().as_ref(),
    ], &program_id);

    
    let make_ix = Instruction{
        accounts:vec![
            AccountMeta::new(maker.pubkey(), true),
            AccountMeta::new(taker.pubkey(), false),
            AccountMeta::new(mint_a.pubkey(), false),
            AccountMeta::new(mint_b.pubkey(), false),
            AccountMeta::new(assc_token_a_maker, false),
            AccountMeta::new(escrow, false),
        ], 
        data:make_ins_data, 
        program_id
    }
}
