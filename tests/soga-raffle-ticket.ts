import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SYSVAR_RENT_PUBKEY,
    SystemProgram,
    Transaction,
    ComputeBudgetProgram
} from "@solana/web3.js";
import {SogaNodeSale} from "../target/types/soga_node_sale";
import {SogaRaffleTicket} from "../target/types/soga_raffle_ticket";
import {PROGRAM_ID as MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    closeAccount,
    closeAccountInstructionData,
    createAssociatedTokenAccount,
    createMint,
    getAssociatedTokenAddress,
    mintToChecked,
    createAssociatedTokenAccountIdempotentInstruction,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token'
import {assert} from "chai";


const SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX: string = "CONFIG";
const TICKET_CONFIG_ACCOUNT_PREFIX: string = "TICKET";
const PAYMENT_CONFIG_ACCOUNT_PREFIX: string = "PAYMENT";
const USER_CONFIG_ACCOUNT_PREFIX: string = "USER";
const USER_PAYMENT_CONFIG_ACCOUNT_PREFIX: string = "USER_PAYMENT";

const SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX: string = "CONFIG";
const SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX: string = "PHASE";
const SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX: string = "PHASE_TIER";
const USER_DETAIL_ACCOUNT_PREFIX: string = "USER";
const USER_TIER_DETAIL_ACCOUNT_PREFIX: string = "USER_TIER";
const SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_ACCOUNT_PREFIX: string = "PHASE_PAYMENT_TOKEN";
const ORDER_DETAIL_ACCOUNT_PREFIX: string = "ORDER";
const COLLECTION_ACCOUNT_PREFIX: string = "COLLECTION";
const NODE_ACCOUNT_PREFIX: string = "NODE";

const mainSigningAuthorityPubKey: PublicKey = anchor.AnchorProvider.env().wallet.publicKey;
const signingAuthorityKeypair: Keypair = Keypair.generate();
const agencyKeypair: Keypair = Keypair.generate();
const kolKeypair: Keypair = Keypair.generate();
const userAKeypair: Keypair = Keypair.generate();
const userBKeypair: Keypair = Keypair.generate();
const feePayerKeypair: Keypair = Keypair.generate();
const priceReceiverKeypair: Keypair = Keypair.generate();
const fullReceiverKeypair: Keypair = Keypair.generate();
const halfReceiverKeypair: Keypair = Keypair.generate();

// Old feed
// const priceFeedSolAddress: PublicKey = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
// const priceFeedUsdtAddress: PublicKey = new PublicKey("38xoQ4oeJCBrcVvca2cGk7iV1dAfrmTR1kmhSCJQ8Jto");
// const priceFeedUsdcAddress: PublicKey = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7");

const priceFeedSolAddress: PublicKey = new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");
const priceFeedIdSol: string = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

const priceFeedUsdtAddress: PublicKey = new PublicKey("HT2PLQBcG5EiCcNSaMHAjSgd9F98ecpATbk4Sk5oYuM");
const priceFeedIdUsdt: string = "2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b";

const priceFeedUsdcAddress: PublicKey = new PublicKey("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX");
const priceFeedIdUsdc: string = "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";

const pythReceiver: PublicKey = new PublicKey("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");
const pythPriceFeed: PublicKey = new PublicKey("pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT");

let tokenMintAccountOne: PublicKey;
let tokenMintAccountTwo: PublicKey;

// PDA and Bumps
let sogaRaffleTicketConfigPDA: PublicKey;
let sogaRaffleTicketConfigBump: number;

let sogaNodeSaleConfigPDA: PublicKey;
let sogaNodeSaleConfigBump: number;

let phaseOne = "one";

let ticketConfigName = "two";

let collection_name: string = "Name";
let collection_symbol: string = "Symbol";
let collection_url: string = "Url";

let nft_name: string = "SogaName";
let nft_symbol: string = "SogaSymbol";
let nft_url: string = "SogaUrl";


describe("soga_raffle_ticket", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const sogaRaffleTicket = anchor.workspace.SogaRaffleTicket as Program<SogaRaffleTicket>;
    const sogaNodeSale = anchor.workspace.SogaNodeSale as Program<SogaNodeSale>;

    const delayTimeCount = 1000;

    let connection = anchor.AnchorProvider.env().connection;


    it("setup signers accounts", async () => {
        await connection.requestAirdrop(signingAuthorityKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("signing authority account: ", signingAuthorityKeypair.publicKey.toBase58());
        console.log("signing authority account sol balance: ", (await connection.getBalance(signingAuthorityKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(agencyKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("agency account: ", agencyKeypair.publicKey.toBase58());
        console.log("agency account sol balance: ", (await connection.getBalance(agencyKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(kolKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("kol account: ", kolKeypair.publicKey.toBase58());
        console.log("kol account sol balance: ", (await connection.getBalance(kolKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(userAKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("user a account: ", userAKeypair.publicKey.toBase58());
        console.log("user a account sol balance: ", (await connection.getBalance(userAKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(userBKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("user b account: ", userBKeypair.publicKey.toBase58());
        console.log("user b account sol balance: ", (await connection.getBalance(userBKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(feePayerKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("fee payer account: ", feePayerKeypair.publicKey.toBase58());
        console.log("fee payer account sol balance: ", (await connection.getBalance(feePayerKeypair.publicKey)) / LAMPORTS_PER_SOL);
    });

    it("create token mint account", async () => {
        tokenMintAccountOne = await createMint(anchor.AnchorProvider.env().connection, signingAuthorityKeypair, signingAuthorityKeypair.publicKey, signingAuthorityKeypair.publicKey, 9)
        console.log("token mint account one: ", tokenMintAccountOne.toBase58());
        await delay(delayTimeCount);

        tokenMintAccountTwo = await createMint(anchor.AnchorProvider.env().connection, signingAuthorityKeypair, signingAuthorityKeypair.publicKey, signingAuthorityKeypair.publicKey, 6)
        console.log("token mint account two: ", tokenMintAccountTwo.toBase58());
        await delay(delayTimeCount);

        // userAPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, userAKeypair.publicKey);
        // console.log("userA PaymentTokenAccount: ", userAPaymentTokenAccount.toBase58());
        //
        // userBPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, userBKeypair.publicKey);
        // console.log("userB PaymentTokenAccount: ", userBPaymentTokenAccount.toBase58());
        //
        // priceReceiverPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, priceReceiverKeypair.publicKey);
        // console.log("priceReceiverPaymentTokenAccount: ", priceReceiverPaymentTokenAccount.toBase58());
        //
        // fullDiscountReceiverPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, fullReceiverKeypair.publicKey);
        // console.log("fullDiscountReceiverPaymentTokenAccount: ", fullDiscountReceiverPaymentTokenAccount.toBase58());
        //
        // halfDiscountPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, halfReceiverKeypair.publicKey);
        // console.log("halfDiscountPaymentTokenAccount: ", halfDiscountPaymentTokenAccount.toBase58());


        await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountOne, // mint
            userAKeypair.publicKey // owner,
        );
        console.log("create user a token signer token account one: ", (await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey)).toBase58());

        await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountTwo, // mint
            userAKeypair.publicKey // owner,
        );
        console.log("create user a token signer token account one: ", (await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey)).toBase58());


        await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountOne, // mint
            userBKeypair.publicKey // owner,
        );
        console.log("create user b token signer token account one: ", (await getAssociatedTokenAddress(tokenMintAccountOne, userBKeypair.publicKey)).toBase58());

        await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountTwo, // mint
            userBKeypair.publicKey // owner,
        );
        console.log("create user B token signer token account one: ", (await getAssociatedTokenAddress(tokenMintAccountTwo, userBKeypair.publicKey)).toBase58());

        let mintTokenToTokenSignerWalletTx = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountOne, // mint
            (await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey)), // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            9 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx: ", mintTokenToTokenSignerWalletTx);

        let mintTokenToTokenSignerWalletTx2 = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountTwo, // mint
            (await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey)), // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            6 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx: ", mintTokenToTokenSignerWalletTx2);

        let mintTokenToTokenSignerWalletTx3 = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountOne, // mint
            (await getAssociatedTokenAddress(tokenMintAccountOne, userBKeypair.publicKey)), // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            9 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx3: ", mintTokenToTokenSignerWalletTx3);

        let mintTokenToTokenSignerWalletTx4 = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            tokenMintAccountTwo, // mint
            (await getAssociatedTokenAddress(tokenMintAccountTwo, userBKeypair.publicKey)), // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            6 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx4: ", mintTokenToTokenSignerWalletTx4);

    });

    it("Create PDA Addresses!", async () => {
        [sogaRaffleTicketConfigPDA, sogaRaffleTicketConfigBump] = getSogaRaffleTicketConfigAccountPdaAndBump(sogaRaffleTicket.programId, SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX);
        console.log("Soga raffle ticket config account pda: ", sogaRaffleTicketConfigPDA.toBase58());

        [sogaNodeSaleConfigPDA, sogaNodeSaleConfigBump] = getSogaNodeSaleConfigAccountPdaAndBump(sogaNodeSale.programId, SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX)
        console.log("soga node sale config account pda: ", sogaNodeSaleConfigPDA.toBase58());

        // [sogaNodeSalePhaseOnePDA, sogaNodeSalePhaseOneBump] = getSogaNodeSalePhaseDetailAccountPdaAndBump(sogaNodeSale.programId, SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX, phaseOne);
        // console.log("soga node sale phase detail account pda: ", sogaNodeSalePhaseOnePDA.toBase58());
    })

    it("initialize - soga raffle ticket", async () => {

        const tx = await sogaRaffleTicket.methods.initialize().accounts({
            feeAndRentPayer: mainSigningAuthorityPubKey,
            mainSigningAuthority: mainSigningAuthorityPubKey,
            config: sogaRaffleTicketConfigPDA,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY
        }).rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("initialize - soga node sale", async () => {

        let sogaNodeSaleConfigPDAInfo = await connection.getAccountInfo(sogaNodeSaleConfigPDA);

        if (sogaNodeSaleConfigPDAInfo == null) {
            const tx = await sogaNodeSale.methods.initialize().accounts({
                payer: mainSigningAuthorityPubKey,
                mainSigningAuthority: mainSigningAuthorityPubKey,
                saleConfig: sogaNodeSaleConfigPDA,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            }).rpc();
            console.log("Your transaction signature", tx);

            await delay(delayTimeCount);

        }
    });

    it("Create ticket sale config - soga raffle ticket", async () => {

        const [ticketConfigPda] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.createTicketConfig(sogaRaffleTicketConfigBump, ticketConfigName, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                mainSigningAuthority: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                config: sogaRaffleTicketConfigPDA,
                ticketConfig: ticketConfigPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());

        assert(ticketConfigPdaData.signingAuthority.toBase58() === signingAuthorityKeypair.publicKey.toBase58(), "1");
        assert(ticketConfigPdaData.ticketPurchaseEnable, "2");
        assert(!ticketConfigPdaData.ticketRefundEnable, "3");
        assert(ticketConfigPdaData.winnerTicketLimit.toNumber() === 2, "4");
    });

    it("Update ticket sale config - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.updateTicketConfig(ticketConfigName, ticketConfigBump, true, true, new BN(3))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());

        assert(ticketConfigPdaData.signingAuthority.toBase58() === signingAuthorityKeypair.publicKey.toBase58(), "1");
        assert(ticketConfigPdaData.ticketPurchaseEnable, "2");
        assert(ticketConfigPdaData.ticketRefundEnable, "3");
        assert(ticketConfigPdaData.winnerTicketLimit.toNumber() === 3, "4");
    });

    it("Create payment config from mint account one - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.createPaymentConfig(ticketConfigName, ticketConfigBump, new BN(6 * LAMPORTS_PER_SOL), new BN(4 * LAMPORTS_PER_SOL))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());

        assert(paymentConfigPdaData.ticketPurchaseEnable, "1");
        assert(paymentConfigPdaData.ticketRefundEnable, "2");
        assert(paymentConfigPdaData.ticketPrice.toNumber() === (new BN(6 * LAMPORTS_PER_SOL)).toNumber(), "3");
        assert(paymentConfigPdaData.refundAmount.toNumber() === (new BN(4 * LAMPORTS_PER_SOL)).toNumber(), "4");
        assert(paymentConfigPdaData.mint.toBase58() === tokenMintAccountOne.toBase58(), "5");

    });

    it("Create payment config from mint account two - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.createPaymentConfig(ticketConfigName, ticketConfigBump, new BN(3 * 1000000), new BN(2 * 1000000))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());

        assert(paymentConfigPdaData.ticketPurchaseEnable, "1");
        assert(paymentConfigPdaData.ticketRefundEnable, "2");
        assert(paymentConfigPdaData.ticketPrice.toNumber() === (new BN(3 * 1000000)).toNumber(), "3");
        assert(paymentConfigPdaData.refundAmount.toNumber() === (new BN(2 * 1000000)).toNumber(), "4");
        assert(paymentConfigPdaData.mint.toBase58() === tokenMintAccountTwo.toBase58(), "5");

    });

    it("update payment config from mint account two - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.updatePaymentConfig(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(6 * 1000000), new BN(4 * 1000000), true, true, true)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                tokenMintAccount: tokenMintAccountTwo,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());

        assert(paymentConfigPdaData.ticketPurchaseEnable, "1");
        assert(paymentConfigPdaData.ticketRefundEnable, "2");
        assert(paymentConfigPdaData.ticketPrice.toNumber() === (new BN(6 * 1000000)).toNumber(), "3");
        assert(paymentConfigPdaData.refundAmount.toNumber() === (new BN(4 * 1000000)).toNumber(), "4");
        assert(paymentConfigPdaData.mint.toBase58() === tokenMintAccountTwo.toBase58(), "5");
        assert(paymentConfigPdaData.enable, "6");
    });

    it("add supply from mint account one - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        let supplyProviderTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey, true);
        console.log("supply provider token account: ", supplyProviderTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.addPaymentSupply(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(LAMPORTS_PER_SOL))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                supplyProvider: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                supplyProviderTokenAccount: supplyProviderTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(LAMPORTS_PER_SOL)).toNumber(), "1");
        assert(paymentConfigPdaData.totalAddedSupply.toNumber() === (new BN(LAMPORTS_PER_SOL)).toNumber(), "2");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 1, "3");
    });

    it("add supply from mint account two - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        let supplyProviderTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey, true);
        console.log("supply provider token account: ", supplyProviderTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.addPaymentSupply(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2 * 1000000))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                supplyProvider: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                supplyProviderTokenAccount: supplyProviderTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(2 * 1000000)).toNumber(), "1");
        assert(paymentConfigPdaData.totalAddedSupply.toNumber() === (new BN(2 * 1000000)).toNumber(), "2");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 2, "3");
    });

    it("Withdraw supply from mint account one - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        let receiverTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey, true);
        console.log("receiver token account: ", receiverTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.withdrawPaymentSupply(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(LAMPORTS_PER_SOL))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                receiver: userAKeypair.publicKey,
                receiverTokenAccount: receiverTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        assert(paymentConfigPdaData.currentBalance.toNumber() === 0, "1");
        assert(paymentConfigPdaData.totalWithdrawSupply.toNumber() === (new BN(LAMPORTS_PER_SOL)).toNumber(), "2");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 0, "3");
    });


    it("withdraw supply from mint account two - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        let receiverTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey, true);
        console.log("receiver token account: ", receiverTokenAccount.toBase58());

        const tx = await sogaRaffleTicket.methods.withdrawPaymentSupply(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2 * 1000000))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                receiver: userAKeypair.publicKey,
                receiverTokenAccount: receiverTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);


        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        assert(paymentConfigPdaData.currentBalance.toNumber() === 0, "1");
        assert(paymentConfigPdaData.totalWithdrawSupply.toNumber() === (new BN(2 * 1000000)).toNumber(), "2");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 0, "3");
    });

    it("buy ticket from mint account one - 1 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userAKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.buyTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                user: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());

        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 2, "1");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 2, "2");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(12 * LAMPORTS_PER_SOL)).toNumber(), "3")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(12 * LAMPORTS_PER_SOL)).toNumber(), "4")

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 2, "5")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "6");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 12, "7");
    });

    it("buy ticket from mint account two - 1 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userAKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.buyTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                user: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 4, "1");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 2, "2");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(12 * 1000000)).toNumber(), "3")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(12 * 1000000)).toNumber(), "4")

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "5")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "6");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 12, "7");
    });

    it("buy ticket from mint account one - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userBKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.buyTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userBKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());

        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 6, "1");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "2");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(24 * LAMPORTS_PER_SOL)).toNumber(), "3")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * LAMPORTS_PER_SOL)).toNumber(), "4")

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 2, "5")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "6");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);
        assert(paymentConfigTokenAccountBalance.value.uiAmount === 24, "7");
    });

    it("buy ticket from mint account two - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userBKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.buyTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userBKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "2");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(24 * 1000000)).toNumber(), "3")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * 1000000)).toNumber(), "4")

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "5")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "6");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 24, "7");
    });

    it("add winner ticket from mint account One - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userBKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda, userPaymentConfigBump] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.addTicketWinner(ticketConfigName, ticketConfigBump, paymentConfigBump, userConfigBump, userPaymentConfigBump, new BN(2))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                tokenMintAccount: tokenMintAccountOne,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 2, "2");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "3");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(24 * LAMPORTS_PER_SOL)).toNumber(), "4")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * LAMPORTS_PER_SOL)).toNumber(), "5")
        assert(paymentConfigPdaData.totalWinnerTicket.toNumber() === 2, "6");


        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "7")
        assert(userConfigPdaData.totalWinTickets.toNumber() === 2, "8")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "9");
        assert(userPaymentConfigPdaData.totalWinTickets.toNumber() === 2, "10");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 24, "11");
    });

    it("add winner ticket from mint account Two - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userBKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda, userPaymentConfigBump] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.addTicketWinner(ticketConfigName, ticketConfigBump, paymentConfigBump, userConfigBump, userPaymentConfigBump, new BN(1))
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                tokenMintAccount: tokenMintAccountTwo,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData)
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "3");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(24 * 1000000)).toNumber(), "4")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * 1000000)).toNumber(), "5")
        assert(paymentConfigPdaData.totalWinnerTicket.toNumber() === 1, "6");


        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "7")
        assert(userConfigPdaData.totalWinTickets.toNumber() === 3, "8")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "9");
        assert(userPaymentConfigPdaData.totalWinTickets.toNumber() === 1, "10");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 24, "11");
    });

    it("refund ticket from mint account Two - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userBKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda, userPaymentConfigBump] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.refundTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, userConfigBump, userPaymentConfigBump)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair, userBKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 1, "3");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData.currentBalance.toNumber())
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "4");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(20 * 1000000)).toNumber(), "5")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * 1000000)).toNumber(), "6")
        assert(paymentConfigPdaData.totalRefund.toNumber() === (new BN(4 * 1000000)).toNumber(), "7")
        assert(paymentConfigPdaData.totalWinnerTicket.toNumber() === 1, "8");
        assert(paymentConfigPdaData.totalTicketRefunded.toNumber() === 1, "9");


        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "10")
        assert(userConfigPdaData.totalWinTickets.toNumber() === 3, "11")
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 1, "12")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "13");
        assert(userPaymentConfigPdaData.totalWinTickets.toNumber() === 1, "14");
        assert(userPaymentConfigPdaData.totalRefundedTickets.toNumber() === 1, "15");
        assert(userPaymentConfigPdaData.totalPurchaseAmount.toNumber() === (new BN(12 * 1000000)).toNumber(), "16");
        assert(userPaymentConfigPdaData.totalRefundAmount.toNumber() === (new BN(4 * 1000000)).toNumber(), "17");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 20, "18");
    });

    it("refund ticket from mint account Two - 1 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountTwo);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountTwo, userAKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userAKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda, userPaymentConfigBump] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.refundTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, userConfigBump, userPaymentConfigBump)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountTwo,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair, userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 3, "3");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData.currentBalance.toNumber())
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "4");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(12 * 1000000)).toNumber(), "5")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * 1000000)).toNumber(), "6")
        assert(paymentConfigPdaData.totalRefund.toNumber() === (new BN(12 * 1000000)).toNumber(), "7")
        assert(paymentConfigPdaData.totalWinnerTicket.toNumber() === 1, "8");
        assert(paymentConfigPdaData.totalTicketRefunded.toNumber() === 3, "9");


        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "10")
        assert(userConfigPdaData.totalWinTickets.toNumber() === 0, "11")
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 2, "12")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "13");
        assert(userPaymentConfigPdaData.totalWinTickets.toNumber() === 0, "14");
        assert(userPaymentConfigPdaData.totalRefundedTickets.toNumber() === 2, "15");
        assert(userPaymentConfigPdaData.totalPurchaseAmount.toNumber() === (new BN(12 * 1000000)).toNumber(), "16");
        assert(userPaymentConfigPdaData.totalRefundAmount.toNumber() === (new BN(8 * 1000000)).toNumber(), "17");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 12, "18");
    });

    it("refund ticket from mint account One - 1 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        const [paymentConfigPda, paymentConfigBump] = getPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, PAYMENT_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, tokenMintAccountOne);
        console.log("Payment Config Pda: ", paymentConfigPda.toBase58());

        const paymentConfigTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, paymentConfigPda, true);
        console.log("Payment Config Token Account: ", paymentConfigTokenAccount.toBase58());

        const userTokenAccount = await getAssociatedTokenAddress(tokenMintAccountOne, userAKeypair.publicKey, true);
        console.log("user token account: ", userTokenAccount.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userAKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());

        let [userPaymentConfigPda, userPaymentConfigBump] = getUserPaymentConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_PAYMENT_CONFIG_ACCOUNT_PREFIX, userConfigPda, paymentConfigPda);
        console.log("user payment config pda: ", userPaymentConfigPda.toBase58());

        const tx = await sogaRaffleTicket.methods.refundTicket(ticketConfigName, ticketConfigBump, paymentConfigBump, userConfigBump, userPaymentConfigBump)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userAKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                paymentConfig: paymentConfigPda,
                paymentConfigTokenAccount: paymentConfigTokenAccount,
                userConfig: userConfigPda,
                userPaymentConfig: userPaymentConfigPda,
                userTokenAccount: userTokenAccount,
                tokenMintAccount: tokenMintAccountOne,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair, userAKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 5, "3");

        let paymentConfigPdaData = await sogaRaffleTicket.account.paymentConfigAccount.fetch(paymentConfigPda.toBase58());
        console.log(paymentConfigPdaData.currentBalance.toNumber())
        assert(paymentConfigPdaData.totalTicketPurchased.toNumber() === 4, "4");
        assert(paymentConfigPdaData.currentBalance.toNumber() === (new BN(16 * LAMPORTS_PER_SOL)).toNumber(), "5")
        assert(paymentConfigPdaData.totalBuy.toNumber() === (new BN(24 * LAMPORTS_PER_SOL)).toNumber(), "6")
        assert(paymentConfigPdaData.totalRefund.toNumber() === (new BN(8 * LAMPORTS_PER_SOL)).toNumber(), "7")
        assert(paymentConfigPdaData.totalWinnerTicket.toNumber() === 2, "8");
        assert(paymentConfigPdaData.totalTicketRefunded.toNumber() === 2, "9");


        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "10")
        assert(userConfigPdaData.totalWinTickets.toNumber() === 0, "11")
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 4, "12")

        let userPaymentConfigPdaData = await sogaRaffleTicket.account.userPaymentConfigAccount.fetch(userPaymentConfigPda.toBase58());
        assert(userPaymentConfigPdaData.totalTickets.toNumber() === 2, "13");
        assert(userPaymentConfigPdaData.totalWinTickets.toNumber() === 0, "14");
        assert(userPaymentConfigPdaData.totalRefundedTickets.toNumber() === 2, "15");
        assert(userPaymentConfigPdaData.totalPurchaseAmount.toNumber() === (new BN(12 * LAMPORTS_PER_SOL)).toNumber(), "16");
        assert(userPaymentConfigPdaData.totalRefundAmount.toNumber() === (new BN(8 * LAMPORTS_PER_SOL)).toNumber(), "17");

        let paymentConfigTokenAccountBalance = await connection.getTokenAccountBalance(paymentConfigTokenAccount);

        assert(paymentConfigTokenAccountBalance.value.uiAmount === 16, "18");
    });

    it("add claim ticket from mint account One - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());


        const tx = await sogaRaffleTicket.methods.addClaimedTicket(ticketConfigName, ticketConfigBump, userConfigBump,)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                userConfig: userConfigPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 5, "3");
        assert(ticketConfigPdaData.totalWinnerClaimedTicket.toNumber() === 1, "4");

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "6");
        assert(userConfigPdaData.totalWinTickets.toNumber() === 3, "7");
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 1, "8");
        assert(userConfigPdaData.totalWinClaimedTickets.toNumber() === 1, "9");
    });

    it("add claim ticket from mint account One - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());


        const tx = await sogaRaffleTicket.methods.addClaimedTicket(ticketConfigName, ticketConfigBump, userConfigBump,)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                userConfig: userConfigPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 5, "3");
        assert(ticketConfigPdaData.totalWinnerClaimedTicket.toNumber() === 2, "4");

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "6");
        assert(userConfigPdaData.totalWinTickets.toNumber() === 3, "7");
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 1, "8");
        assert(userConfigPdaData.totalWinClaimedTickets.toNumber() === 2, "9");
    });

    it("add claim ticket from mint account One - 2 - soga raffle ticket", async () => {
        const [ticketConfigPda, ticketConfigBump] = getTicketSaleConfigAccountPdaAndBump(sogaRaffleTicket.programId, TICKET_CONFIG_ACCOUNT_PREFIX, ticketConfigName);
        console.log("Ticket Config Pda: ", ticketConfigPda.toBase58());

        let [userConfigPda, userConfigBump] = getUserConfigAccountPdaAndBump(sogaRaffleTicket.programId, USER_CONFIG_ACCOUNT_PREFIX, ticketConfigPda, userBKeypair.publicKey);
        console.log("user config pda: ", userConfigPda.toBase58());


        const tx = await sogaRaffleTicket.methods.addClaimedTicket(ticketConfigName, ticketConfigBump, userConfigBump,)
            .accounts({
                feeAndRentPayer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userBKeypair.publicKey,
                ticketConfig: ticketConfigPda,
                userConfig: userConfigPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        let ticketConfigPdaData = await sogaRaffleTicket.account.ticketConfigAccount.fetch(ticketConfigPda.toBase58());
        assert(ticketConfigPdaData.totalTicketPurchased.toNumber() === 8, "1");
        assert(ticketConfigPdaData.totalWinnerTicket.toNumber() === 3, "2");
        assert(ticketConfigPdaData.totalTicketRefunded.toNumber() === 5, "3");
        assert(ticketConfigPdaData.totalWinnerClaimedTicket.toNumber() === 3, "4");

        let userConfigPdaData = await sogaRaffleTicket.account.userConfigAccount.fetch(userConfigPda.toBase58());
        assert(userConfigPdaData.totalTickets.toNumber() === 4, "6");
        assert(userConfigPdaData.totalWinTickets.toNumber() === 3, "7");
        assert(userConfigPdaData.totalRefundedTickets.toNumber() === 1, "8");
        assert(userConfigPdaData.totalWinClaimedTickets.toNumber() === 3, "9");
    });

    // it("Remove Events", async () => {
    // });

});


function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

function getMetadataPda(mint: PublicKey): PublicKey {
    const [metadataPda] = PublicKey.findProgramAddressSync(
        [
            Buffer.from('metadata'),
            MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
        ], MPL_TOKEN_METADATA_PROGRAM_ID);

    return metadataPda;
}

function getMasterPda(mint: PublicKey): PublicKey {
    const [masterPda] = PublicKey.findProgramAddressSync(
        [
            Buffer.from('metadata'),
            MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from('edition')
        ],
        MPL_TOKEN_METADATA_PROGRAM_ID
    );

    return masterPda;
}

function getSogaRaffleTicketConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(prefix)],
        programAddress
    )
}

function getTicketSaleConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string, ticketConfigName: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(ticketConfigName),
        ],
        programAddress
    )
}


function getPaymentConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string, ticketConfigPda: PublicKey, tokenMintAccount: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            ticketConfigPda.toBuffer(),
            tokenMintAccount.toBuffer(),
        ],
        programAddress
    )
}


function getUserConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string, ticketConfigPda: PublicKey, user: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            ticketConfigPda.toBuffer(),
            user.toBuffer(),
        ],
        programAddress
    )
}

function getUserPaymentConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string, userConfigPda: PublicKey, paymentConfigPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            userConfigPda.toBuffer(),
            paymentConfigPda.toBuffer(),
        ],
        programAddress
    )
}

////////////////////////////
function getSogaNodeSaleConfigAccountPdaAndBump(programAddress: PublicKey, prefix: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(prefix)],
        programAddress
    )
}

function getSogaNodeSalePhaseDetailAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName)
        ],
        programAddress
    )
}

function getSogaNodeSalePhaseDetailTierAccountPdaAndBump(programAddress: PublicKey, prefix: string, tier: string,
                                                         sogaNodeSalePhaseDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            Buffer.from(tier),
        ],
        programAddress
    )
}

function getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(programAddress: PublicKey, prefix: string,
                                                                   sogaNodeSalePhaseTierDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            sogaNodeSalePhaseTierDetailPda.toBuffer()
        ],
        programAddress
    )
}

function getUserAccountPdaAndBump(programAddress: PublicKey, prefix: string,
                                  sogaNodeSalePhaseDetailPda: PublicKey, userAddress: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            userAddress.toBuffer()
        ],
        programAddress
    )
}

function getUserTierAccountPdaAndBump(programAddress: PublicKey, prefix: string,
                                      userDetailPdaAddress: PublicKey,
                                      sogaNodeSalePhaseTierDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            userDetailPdaAddress.toBuffer(),
            sogaNodeSalePhaseTierDetailPda.toBuffer()
        ],
        programAddress
    )
}

function getNodeMintAccount(programAddress: PublicKey, prefix: string,
                            collectionMintAccount: PublicKey, tokenId: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            collectionMintAccount.toBuffer(),
            Buffer.from(tokenId)
        ],
        programAddress
    )
}

function getSogaNodeSalePhasePaymentTokenDetailAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string,
                                                                 sogaNodeSalePhaseDetailPda: PublicKey, mintAccount: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            mintAccount.toBuffer(),
        ],
        programAddress
    )
}

function getOrderDetailAccountPdaAndBump(programAddress: PublicKey, prefix: string,
                                         sogaNodeSalePhaseDetailPda: PublicKey,
                                         userDetail: PublicKey, orderId: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            userDetail.toBuffer(),
            Buffer.from(orderId)
        ],
        programAddress
    )
}