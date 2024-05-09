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

interface InitializeSalePhaseEvent {
    timestamp: BN,

    salePhaseName: string,

    totalTiers: number,

    signingAuthority: PublicKey,

    priceFeed: PublicKey,

    paymentReceiver: PublicKey
}


interface InitializeSalePhaseTierEvent {
    timestamp: BN,

    salePhaseName: string,

    tierId: string,

    collectionMintAddress: PublicKey,

    price: BN,

    quantity: BN,

    mintLimit: BN,
}

interface UpdateSalePhaseEvent {
    timestamp: BN,

    salePhaseName: string,

    priceFeed: PublicKey,

    paymentReceiver: PublicKey,

    buyEnable: boolean,

    airdropEnable: boolean,
}

interface UpdateSalePhaseTierEvent {
    timestamp: BN,

    salePhaseName: string,

    tierId: string,

    price: BN,

    mintLimit: BN,

    buyEnable: boolean,

    airdropEnable: boolean,
}

interface AirdropEvent {
    timestamp: BN,

    salePhaseName: string,

    tierId: string,

    tokenId: string,

    collectionMintAccount: PublicKey,

    nodeMintAccount: PublicKey,

    user: PublicKey,
}

interface BuyEvent {
    timestamp: BN,

    salePhaseName: string,

    tierId: string,

    tokenId: string,

    collectionMintAccount: PublicKey,

    nodeMintAccount: PublicKey,

    user: PublicKey,

    priceFeed: PublicKey,

    paymentReceiver: PublicKey,

    fullDiscountReceiver: PublicKey,

    halfDiscountReceiver: PublicKey,

    totalPriceInLamport: BN,

    subPriceInLamport: BN,

    fullDiscountInLamport: BN,

    halfDiscountInLamport: BN,

    priceInUsd: BN,

    pythPrice: BN,

    pythExpo: BN,

    allowFullDiscount: boolean,

    fullDiscount: BN,

    allowHalfDiscount: boolean,

    halfDiscount: BN
}

//// Event Name
const InitializeSalePhaseEventName = "InitializeSalePhaseEvent";
const InitializeSalePhaseTierEventName = "InitializeSalePhaseTierEvent";
const UpdateSalePhaseEventName = "UpdateSalePhaseEvent";
const UpdateSalePhaseTierEventName = "UpdateSalePhaseTierEvent";
const BuyEventName = "BuyEvent";
const AirdropEventName = "AirdropEvent";

const handleInitializeSalePhaseEvent = (ev: InitializeSalePhaseEvent) =>
    console.log(`${InitializeSalePhaseEventName} ==> `, ev);

const handleInitializeSalePhaseTierEvent = (ev: InitializeSalePhaseTierEvent) =>
    console.log(`${InitializeSalePhaseTierEventName} ==> `, ev);

const handleUpdateSalePhaseEvent = (ev: UpdateSalePhaseEvent) =>
    console.log(`${UpdateSalePhaseEventName} ==> `, ev);

const handleUpdateSalePhaseTierEvent = (ev: UpdateSalePhaseTierEvent) =>
    console.log(`${UpdateSalePhaseTierEventName} ==> `, ev);

const handleBuyEvent = (ev: BuyEvent) =>
    console.log(`${BuyEventName} ==> `, ev);

const handleAirdropEvent = (ev: AirdropEvent) =>
    console.log(`${AirdropEventName} ==> `, ev);


const SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX: string = "CONFIG";
const SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX: string = "PHASE";
const SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX: string = "PHASE_TIER";
const USER_DETAIL_ACCOUNT_PREFIX: string = "USER";
const USER_TIER_DETAIL_ACCOUNT_PREFIX: string = "USER_TIER";
const SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_ACCOUNT_PREFIX: string = "PHASE_PAYMENT_TOKEN";
const ORDER_DETAIL_ACCOUNT_PREFIX: string = "ORDER";

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

const priceFeedSolAddress: PublicKey = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
const priceFeedUsdtAddress: PublicKey = new PublicKey("38xoQ4oeJCBrcVvca2cGk7iV1dAfrmTR1kmhSCJQ8Jto");
const priceFeedUsdcAddress: PublicKey = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7");

let paymentTokenMintAccount: PublicKey;

let userAPaymentTokenAccount: PublicKey;
let userBPaymentTokenAccount: PublicKey;
let priceReceiverPaymentTokenAccount: PublicKey;
let fullDiscountReceiverPaymentTokenAccount: PublicKey;
let halfDiscountPaymentTokenAccount: PublicKey;

// PDA and Bumps

let sogaNodeSaleConfigPDA: PublicKey;
let sogaNodeSaleConfigBump: number;

let sogaNodeSalePhaseOnePDA: PublicKey;
let sogaNodeSalePhaseOneBump: number;

let phaseOne = "one";
let phaseTwo = "two";

let collection_name: string = "Name";
let collection_symbol: string = "Symbol";
let collection_url: string = "Url";

let nft_name: string = "SogaName";
let nft_symbol: string = "SogaSymbol";
let nft_url: string = "SogaUrl";

describe("soga_node_sale", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SogaNodeSale as Program<SogaNodeSale>;

    const delayTimeCount = 1000;

    let connection = anchor.AnchorProvider.env().connection;

    // Setup Event
    const initializeSalePhaseEventListener = program.addEventListener(InitializeSalePhaseEventName, handleInitializeSalePhaseEvent);
    const initializeSalePhaseTierEventListener = program.addEventListener(InitializeSalePhaseTierEventName, handleInitializeSalePhaseTierEvent);
    const updateSalePhaseEventListener = program.addEventListener(UpdateSalePhaseEventName, handleUpdateSalePhaseEvent);
    const updateSalePhaseTierEventListener = program.addEventListener(UpdateSalePhaseTierEventName, handleUpdateSalePhaseTierEvent);
    // const buyEventListener = program.addEventListener(BuyEventName, handleBuyEvent);
    const airdropEventListener = program.addEventListener(AirdropEventName, handleAirdropEvent);

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
        paymentTokenMintAccount = await createMint(anchor.AnchorProvider.env().connection, signingAuthorityKeypair, signingAuthorityKeypair.publicKey, signingAuthorityKeypair.publicKey, 9)
        console.log("payment token mint account: ", paymentTokenMintAccount.toBase58());
        await delay(delayTimeCount);

        userAPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, userAKeypair.publicKey);
        console.log("userA PaymentTokenAccount: ", userAPaymentTokenAccount.toBase58());

        userBPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, userBKeypair.publicKey);
        console.log("userB PaymentTokenAccount: ", userBPaymentTokenAccount.toBase58());

        priceReceiverPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, priceReceiverKeypair.publicKey);
        console.log("priceReceiverPaymentTokenAccount: ", priceReceiverPaymentTokenAccount.toBase58());

        fullDiscountReceiverPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, fullReceiverKeypair.publicKey);
        console.log("fullDiscountReceiverPaymentTokenAccount: ", fullDiscountReceiverPaymentTokenAccount.toBase58());

        halfDiscountPaymentTokenAccount = await getAssociatedTokenAddress(paymentTokenMintAccount, halfReceiverKeypair.publicKey);
        console.log("halfDiscountPaymentTokenAccount: ", halfDiscountPaymentTokenAccount.toBase58());


        let createUserATokenAccount = await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            paymentTokenMintAccount, // mint
            userAKeypair.publicKey // owner,
        );
        console.log("create user a token signer token account: ", createUserATokenAccount.toBase58());

        let createUserBTokenAccount = await createAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            paymentTokenMintAccount, // mint
            userBKeypair.publicKey // owner,
        );
        console.log("create user b token signer token account: ", createUserBTokenAccount.toBase58());

        let mintTokenToTokenSignerWalletTx = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            paymentTokenMintAccount, // mint
            userAPaymentTokenAccount, // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            9 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx: ", mintTokenToTokenSignerWalletTx);

        let mintTokenToTokenSignerWalletTx2 = await mintToChecked(
            anchor.AnchorProvider.env().connection, // connection
            signingAuthorityKeypair, // fee payer
            paymentTokenMintAccount, // mint
            userBPaymentTokenAccount, // receiver (sholud be a token account)
            signingAuthorityKeypair, // mint authority
            (20000 * anchor.web3.LAMPORTS_PER_SOL), // amount. if your decimals is 8, you mint 10^8 for 1 token.
            9 // decimals
        );
        console.log("mintTokenToTokenSignerWalletTx: ", mintTokenToTokenSignerWalletTx2);

    });

    it("Create PDA Addresses!", async () => {
        [sogaNodeSaleConfigPDA, sogaNodeSaleConfigBump] = getSogaNodeSaleConfigAccountPdaAndBump(program.programId, SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX)
        console.log("soga node sale config account pda: ", sogaNodeSaleConfigPDA.toBase58());
        console.log("soga node sale config account bump: ", sogaNodeSaleConfigBump);

        [sogaNodeSalePhaseOnePDA, sogaNodeSalePhaseOneBump] = getSogaNodeSalePhaseDetailAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX, phaseOne);
        console.log("soga node sale phase detail account pda: ", sogaNodeSalePhaseOnePDA.toBase58());
        console.log("soga node sale phase detail account bump: ", sogaNodeSalePhaseOneBump);
    })

    it("initialize", async () => {

        const tx = await program.methods.initialize().accounts({
            payer: mainSigningAuthorityPubKey,
            mainSigningAuthority: mainSigningAuthorityPubKey,
            saleConfig: sogaNodeSaleConfigPDA,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY
        }).rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("Initialize Sale Phase One", async () => {

        const tx = await program.methods.initializeSalePhase(sogaNodeSaleConfigBump, phaseOne, 5, nft_name, nft_symbol, nft_url)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                mainSigningAuthority: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                saleConfig: sogaNodeSaleConfigPDA,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                priceFeed: priceFeedSolAddress,
                paymentReceiver: priceReceiverKeypair.publicKey,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        assert(salePhaseData.signingAuthority.toBase58() === signingAuthorityKeypair.publicKey.toBase58());
        assert(salePhaseData.paymentReceiver.toBase58() === priceReceiverKeypair.publicKey.toBase58());
        assert(salePhaseData.priceFeedAddress.toBase58() === priceFeedSolAddress.toBase58());


        assert(salePhaseData.totalPayment.toString() === "0");
        assert(salePhaseData.totalDiscount.toString() === "0");
        assert(salePhaseData.totalPaymentReceived.toString() === "0");

        assert(salePhaseData.totalMint.toString() === "0");
        assert(salePhaseData.totalBuy.toString() === "0");
        assert(salePhaseData.totalAirdrop.toString() === "0");

        assert(salePhaseData.buyEnable);
        assert(salePhaseData.airdropEnable);

        assert(salePhaseData.totalTiers.toString() === "5");
        assert(salePhaseData.totalInitializeTiers.toString() === "0");
        assert(salePhaseData.totalCompletedTiers.toString() === "0");

        assert(salePhaseData.name === nft_name);
        assert(salePhaseData.symbol === nft_symbol);
        assert(salePhaseData.metadataBaseUri === nft_url);
    });

    it("Update Sale Phase One", async () => {

        const tx = await program.methods.updateSalePhase(sogaNodeSalePhaseOneBump, phaseOne, nft_name, nft_symbol, nft_url, true, true, true)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                priceFeed: priceFeedSolAddress,
                paymentReceiver: priceReceiverKeypair.publicKey,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        assert(salePhaseData.signingAuthority.toBase58() === signingAuthorityKeypair.publicKey.toBase58());
        assert(salePhaseData.paymentReceiver.toBase58() === priceReceiverKeypair.publicKey.toBase58());
        assert(salePhaseData.priceFeedAddress.toBase58() === priceFeedSolAddress.toBase58());


        assert(salePhaseData.totalPayment.toString() === "0");
        assert(salePhaseData.totalDiscount.toString() === "0");
        assert(salePhaseData.totalPaymentReceived.toString() === "0");

        assert(salePhaseData.totalMint.toString() === "0");
        assert(salePhaseData.totalBuy.toString() === "0");
        assert(salePhaseData.totalAirdrop.toString() === "0");

        assert(salePhaseData.buyEnable);
        assert(salePhaseData.airdropEnable);

        assert(salePhaseData.totalTiers.toString() === "5");
        assert(salePhaseData.totalInitializeTiers.toString() === "0");
        assert(salePhaseData.totalCompletedTiers.toString() === "0");

        assert(salePhaseData.name === nft_name);
        assert(salePhaseData.symbol === nft_symbol);
        assert(salePhaseData.metadataBaseUri === nft_url);
    });

    it("initialize Sale Phase payment token usdt", async () => {

        const [salePhasePaymentTokenDetailPda] = getSogaNodeSalePhasePaymentTokenDetailAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_ACCOUNT_PREFIX,
            phaseOne, sogaNodeSalePhaseOnePDA, paymentTokenMintAccount);


        const tx = await program.methods.initializeSalePhaseTokenPayment(sogaNodeSalePhaseOneBump, phaseOne)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                priceFeed: priceFeedUsdtAddress,
                salePhasePaymentTokenDetail: salePhasePaymentTokenDetailPda,
                paymentTokenMintAccount: paymentTokenMintAccount,
                paymentTokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("update Sale Phase payment token usdt", async () => {

        const [salePhasePaymentTokenDetailPda, salePhasePaymentTokenDetailBump] = getSogaNodeSalePhasePaymentTokenDetailAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_ACCOUNT_PREFIX,
            phaseOne, sogaNodeSalePhaseOnePDA, paymentTokenMintAccount);

        const tx = await program.methods.updateSalePhaseTokenPayment(sogaNodeSalePhaseOneBump, salePhasePaymentTokenDetailBump, phaseOne, true)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                priceFeed: priceFeedUsdtAddress,
                salePhasePaymentTokenDetail: salePhasePaymentTokenDetailPda,
                paymentTokenMintAccount: paymentTokenMintAccount,
                paymentTokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("Initialize Sale Phase One Tier One", async () => {

        const tierId = 1;

        const [nodeSalePhaseTierPds] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
        console.log("Node Sale Phase one tier one: ", nodeSalePhaseTierPds.toBase58());

        const [nodeSalePhaseTierCollectionPda] = getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA, nodeSalePhaseTierPds);
        console.log("Node Sale Phase one tier one collection: ", nodeSalePhaseTierCollectionPda.toBase58());

        const nodeSalePhaseTierCollectionMasterPda = getMasterPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier one collection Master: ", nodeSalePhaseTierCollectionMasterPda.toBase58());

        const nodeSalePhaseTierCollectionMetadataPda = getMetadataPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier one collection Metadata: ", nodeSalePhaseTierCollectionMetadataPda.toBase58());

        const nodeSalePhaseTierCollectionTokenAccount = await getAssociatedTokenAddress(nodeSalePhaseTierCollectionPda, nodeSalePhaseTierPds, true, TOKEN_PROGRAM_ID);
        console.log("Node Sale Phase one tier one collection token account: ", nodeSalePhaseTierCollectionTokenAccount.toBase58());

        const tx = await program.methods.initializeSalePhaseTier(sogaNodeSalePhaseOneBump, phaseOne, tierId.toString(), new BN(500), new BN(10), new BN(2),
            collection_name, collection_symbol, collection_url)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                salePhaseTierDetail: nodeSalePhaseTierPds,
                collectionMintAccount: nodeSalePhaseTierCollectionPda,
                collectionMasterEdition: nodeSalePhaseTierCollectionMasterPda,
                collectionMetadata: nodeSalePhaseTierCollectionMetadataPda,
                collectionTokenAccount: nodeSalePhaseTierCollectionTokenAccount,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        const newSalePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        assert(newSalePhaseData.totalInitializeTiers === 1);

        const salePhaseTierData = await program.account.sogaNodeSalePhaseTierDetailAccount.fetch(nodeSalePhaseTierPds.toBase58());

        // console.log(salePhaseTierData)

        assert(salePhaseTierData.collectionMintAddress.toBase58() === nodeSalePhaseTierCollectionPda.toBase58());
        assert(salePhaseTierData.buyEnable);
        assert(salePhaseTierData.airdropEnable);
        assert(!salePhaseTierData.isCompleted);
        // assert(salePhaseTierData.price.toString() === (new BN(3000)).toString());
        assert(salePhaseTierData.quantity.toString() === (new BN(10)).toString());
        assert(salePhaseTierData.mintLimit.toString() === (new BN(2)).toString());
    });

    it("Update Sale Phase One Tier One", async () => {
        const tierId = 1;

        const [nodeSalePhaseTierPds, nodeSalePhaseTierBump] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
        console.log("Node Sale Phase one tier one: ", nodeSalePhaseTierPds.toBase58());

        const tx = await program.methods.updateSalePhaseTier(sogaNodeSalePhaseOneBump, nodeSalePhaseTierBump, phaseOne, tierId.toString(), new BN(500), new BN(2),
            true, true, true)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                salePhaseTierDetail: nodeSalePhaseTierPds,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        const newSalePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        assert(newSalePhaseData.totalInitializeTiers === 1);

        const salePhaseTierData = await program.account.sogaNodeSalePhaseTierDetailAccount.fetch(nodeSalePhaseTierPds.toBase58());

        // console.log(salePhaseTierData)

        assert(salePhaseTierData.buyEnable);
        assert(salePhaseTierData.airdropEnable);
        assert(!salePhaseTierData.isCompleted);
        // assert(salePhaseTierData.price.toString() === (new BN(3000)).toString());
        assert(salePhaseTierData.quantity.toString() === (new BN(10)).toString());
        assert(salePhaseTierData.mintLimit.toString() === (new BN(2)).toString());
    });

    it("Initialize Sale Phase One Tier Two", async () => {

        const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        const tierId = (salePhaseData.totalInitializeTiers + 1);

        console.log(tierId)

        assert(tierId <= salePhaseData.totalTiers, "invalid tier Id");

        const [nodeSalePhaseTierPda] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
        console.log("Node Sale Phase one tier two: ", nodeSalePhaseTierPda.toBase58());

        const [nodeSalePhaseTierCollectionPda] = getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA, nodeSalePhaseTierPda);
        console.log("Node Sale Phase one tier two collection: ", nodeSalePhaseTierCollectionPda.toBase58());

        const nodeSalePhaseTierCollectionMasterPda = getMasterPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier two collection Master: ", nodeSalePhaseTierCollectionMasterPda.toBase58());

        const nodeSalePhaseTierCollectionMetadataPda = getMetadataPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier two collection Metadata: ", nodeSalePhaseTierCollectionMetadataPda.toBase58());

        const nodeSalePhaseTierCollectionTokenAccount = await getAssociatedTokenAddress(nodeSalePhaseTierCollectionPda, nodeSalePhaseTierPda, true, TOKEN_PROGRAM_ID);
        console.log("Node Sale Phase one tier two collection token account: ", nodeSalePhaseTierCollectionTokenAccount.toBase58());

        const tx = await program.methods.initializeSalePhaseTier(sogaNodeSalePhaseOneBump, phaseOne, tierId.toString(), new BN(2 * LAMPORTS_PER_SOL), new BN(10), new BN(2),
            collection_name, collection_symbol, collection_url)
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                salePhaseTierDetail: nodeSalePhaseTierPda,
                collectionMintAccount: nodeSalePhaseTierCollectionPda,
                collectionMasterEdition: nodeSalePhaseTierCollectionMasterPda,
                collectionMetadata: nodeSalePhaseTierCollectionMetadataPda,
                collectionTokenAccount: nodeSalePhaseTierCollectionTokenAccount,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([signingAuthorityKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);

        const newSalePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        assert(newSalePhaseData.totalInitializeTiers === 2);

        const salePhaseTierData = await program.account.sogaNodeSalePhaseTierDetailAccount.fetch(nodeSalePhaseTierPda.toBase58());

        console.log(salePhaseTierData)

        assert(salePhaseTierData.collectionMintAddress.toBase58() === nodeSalePhaseTierCollectionPda.toBase58());
        assert(salePhaseTierData.buyEnable);
        assert(salePhaseTierData.airdropEnable);
        assert(!salePhaseTierData.isCompleted);
        assert(salePhaseTierData.price.toString() === (new BN(LAMPORTS_PER_SOL * 2)).toString());
        assert(salePhaseTierData.quantity.toString() === (new BN(10)).toString());
        assert(salePhaseTierData.mintLimit.toString() === (new BN(2)).toString());
    });

    // it("Buy Node One Sale Phase One Tier One", async () => {
    //
    //     const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());
    //
    //     const tierId: number = 1;
    //     const tokenIdId: number = 1;
    //
    //     const [nodeSalePhaseTierPda, nodeSalePhaseTierBump] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    //         phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
    //     console.log("Node Sale Phase one tier two: ", nodeSalePhaseTierPda.toBase58());
    //
    //     const [nodeSalePhaseTierCollectionPda, nodeSalePhaseTierCollectionBump] = getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    //         phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA, nodeSalePhaseTierPda);
    //     console.log("Node Sale Phase one tier two collection: ", nodeSalePhaseTierCollectionPda.toBase58());
    //
    //     const nodeSalePhaseTierCollectionMasterPda = getMasterPda(nodeSalePhaseTierCollectionPda);
    //     console.log("Node Sale Phase one tier two collection Master: ", nodeSalePhaseTierCollectionMasterPda.toBase58());
    //
    //     const nodeSalePhaseTierCollectionMetadataPda = getMetadataPda(nodeSalePhaseTierCollectionPda);
    //     console.log("Node Sale Phase one tier two collection Metadata: ", nodeSalePhaseTierCollectionMetadataPda.toBase58());
    //     //
    //     // const nodeSalePhaseTierCollectionTokenAccount = await getAssociatedTokenAddress(nodeSalePhaseTierCollectionPda, nodeSalePhaseTierPda, true, TOKEN_PROGRAM_ID);
    //     // console.log("Node Sale Phase one tier two collection token account: ", nodeSalePhaseTierCollectionTokenAccount.toBase58());
    //
    //     const [userDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey);
    //     console.log("User detail pda: ", userDetailPda.toBase58());
    //
    //     const [userPhaseTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey, userDetailPda, tierId.toString(), nodeSalePhaseTierPda);
    //     console.log("User tier detail pda: ", userPhaseTierDetailPda.toBase58());
    //
    //     const [nodeMintAccountPda] = getNodeMintAccount(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    //         phaseOne, sogaNodeSalePhaseOnePDA, tierId.toString(), nodeSalePhaseTierPda, nodeSalePhaseTierCollectionPda, tokenIdId.toString());
    //     console.log("Node Mint Account Pda: ", nodeMintAccountPda.toBase58());
    //
    //     const nodeMintAccountMasterPda = getMasterPda(nodeMintAccountPda);
    //     console.log("node mint account master pda: ", nodeMintAccountMasterPda.toBase58());
    //
    //     const nodeMintAccountMetadataPda = getMetadataPda(nodeMintAccountPda)
    //     console.log("node mint account metadata pda: ", nodeMintAccountMetadataPda.toBase58());
    //
    //
    //     const nodeUserTokenAccount = await getAssociatedTokenAddress(nodeMintAccountPda, userAKeypair.publicKey, true, TOKEN_PROGRAM_ID);
    //     console.log("node user token account: ", nodeUserTokenAccount.toBase58());
    //
    //     const [fullDiscountReceiverUserDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, fullReceiverKeypair.publicKey);
    //     const [fullDiscountReceiverUserTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, fullReceiverKeypair.publicKey, fullDiscountReceiverUserDetailPda, tierId.toString(), nodeSalePhaseTierPda);
    //
    //     const [halfDiscountReceiverUserDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, halfReceiverKeypair.publicKey);
    //     const [halfDiscountReceiverUserTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, halfReceiverKeypair.publicKey, halfDiscountReceiverUserDetailPda, tierId.toString(), nodeSalePhaseTierPda);
    //
    //
    //     const tx = await program.methods.buy(sogaNodeSalePhaseOneBump, nodeSalePhaseTierBump, nodeSalePhaseTierCollectionBump,
    //         phaseOne, tierId.toString(), tokenIdId.toString(), true, new BN(20), true, new BN(10))
    //         .accounts({
    //             payer: mainSigningAuthorityPubKey,
    //             signingAuthority: signingAuthorityKeypair.publicKey,
    //             user: userAKeypair.publicKey,
    //             salePhaseDetail: sogaNodeSalePhaseOnePDA,
    //             salePhaseTierDetail: nodeSalePhaseTierPda,
    //             userDetail: userDetailPda,
    //             userTierDetail: userPhaseTierDetailPda,
    //             collectionMintAccount: nodeSalePhaseTierCollectionPda,
    //             nodeMintAccount: nodeMintAccountPda,
    //             userTokenAccount: nodeUserTokenAccount,
    //             tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //             systemProgram: SystemProgram.programId,
    //             rent: SYSVAR_RENT_PUBKEY
    //         }).remainingAccounts([
    //             {
    //                 pubkey: priceFeedSolAddress,
    //                 isWritable: false,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: priceReceiverKeypair.publicKey,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: fullReceiverKeypair.publicKey,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: halfReceiverKeypair.publicKey,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: nodeSalePhaseTierCollectionMetadataPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: nodeSalePhaseTierCollectionMasterPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: nodeMintAccountMetadataPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: nodeMintAccountMasterPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: fullDiscountReceiverUserDetailPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: fullDiscountReceiverUserTierDetailPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: halfDiscountReceiverUserDetailPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             },
    //             {
    //                 pubkey: halfDiscountReceiverUserTierDetailPda,
    //                 isWritable: true,
    //                 isSigner: false
    //             }
    //         ]).preInstructions([ComputeBudgetProgram.setComputeUnitLimit({units: 1400_000})])
    //         .signers([signingAuthorityKeypair, userAKeypair])
    //         .rpc({});
    //
    //     console.log("Your transaction signature", tx);
    //
    //     await delay(delayTimeCount);
    //
    //     console.log("full account sol balance: ", (await connection.getBalance(fullReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
    //     console.log("half account sol balance: ", (await connection.getBalance(halfReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
    //     console.log("receive account sol balance: ", (await connection.getBalance(priceReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
    // });

    it("Buy with token Node One Sale Phase One Tier One", async () => {

        const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        const tierId: number = 1;
        const tokenIdId: number = 1;

        const [nodeSalePhaseTierPda, nodeSalePhaseTierBump] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
        console.log("Node Sale Phase one tier two: ", nodeSalePhaseTierPda.toBase58());

        const [userDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey);
        console.log("User detail pda: ", userDetailPda.toBase58());

        const [userPhaseTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey, userDetailPda, tierId.toString(), nodeSalePhaseTierPda);
        console.log("User tier detail pda: ", userPhaseTierDetailPda.toBase58());

        const [orderPda] = getOrderDetailAccountPdaAndBump(program.programId, ORDER_DETAIL_ACCOUNT_PREFIX, sogaNodeSalePhaseOnePDA, userDetailPda, "1");


        const [fullDiscountReceiverUserDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, fullReceiverKeypair.publicKey);
        const [fullDiscountReceiverUserTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, fullReceiverKeypair.publicKey, fullDiscountReceiverUserDetailPda, tierId.toString(), nodeSalePhaseTierPda);

        const [halfDiscountReceiverUserDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, halfReceiverKeypair.publicKey);
        const [halfDiscountReceiverUserTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, halfReceiverKeypair.publicKey, halfDiscountReceiverUserDetailPda, tierId.toString(), nodeSalePhaseTierPda);

        const [salePhasePaymentTokenDetailPda, salePhasePaymentTokenDetailBump] = getSogaNodeSalePhasePaymentTokenDetailAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_ACCOUNT_PREFIX,
            phaseOne, sogaNodeSalePhaseOnePDA, paymentTokenMintAccount);

        let createPriceReceiverTokenAccount = createAssociatedTokenAccountIdempotentInstruction(
            mainSigningAuthorityPubKey,
            priceReceiverPaymentTokenAccount,
            priceReceiverKeypair.publicKey,
            paymentTokenMintAccount,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        let createFullDiscountTokenAccount = createAssociatedTokenAccountIdempotentInstruction(
            mainSigningAuthorityPubKey,
            fullDiscountReceiverPaymentTokenAccount,
            fullReceiverKeypair.publicKey,
            paymentTokenMintAccount,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        let createHalfDiscountTokenAccount = createAssociatedTokenAccountIdempotentInstruction(
            mainSigningAuthorityPubKey,
            halfDiscountPaymentTokenAccount,
            halfReceiverKeypair.publicKey,
            paymentTokenMintAccount,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const tx = await program.methods.buyWithToken(sogaNodeSalePhaseOneBump, nodeSalePhaseTierBump,
            phaseOne, tierId.toString(), tokenIdId.toString(), "1", new BN(2), true, new BN(20), true, new BN(10))
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userAKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                salePhaseTierDetail: nodeSalePhaseTierPda,
                userDetail: userDetailPda,
                userTierDetail: userPhaseTierDetailPda,
                orderDetail: orderPda,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            }).remainingAccounts([
                {
                    pubkey: priceFeedUsdtAddress, // 0
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: priceReceiverKeypair.publicKey, // 1
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: fullReceiverKeypair.publicKey, // 2
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: halfReceiverKeypair.publicKey, // 3
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: salePhasePaymentTokenDetailPda, // 4
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: paymentTokenMintAccount, // 5
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: TOKEN_PROGRAM_ID, // 6
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: userAPaymentTokenAccount, // 7
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: priceReceiverPaymentTokenAccount, // 8
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: fullDiscountReceiverPaymentTokenAccount, // 9
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: halfDiscountPaymentTokenAccount, // 10
                    isWritable: true,
                    isSigner: false
                }
            ]).preInstructions([
                ComputeBudgetProgram.setComputeUnitLimit({units: 1400_000}),
                createPriceReceiverTokenAccount,
                createFullDiscountTokenAccount,
                createHalfDiscountTokenAccount
            ])
            .signers([signingAuthorityKeypair, userAKeypair])
            .rpc({});

        // console.log("Your transaction signature", tx);
        //
        // await delay(delayTimeCount);
        //
        // console.log("full account sol balance: ", (await connection.getBalance(fullReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
        // console.log("half account sol balance: ", (await connection.getBalance(halfReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
        // console.log("receive account sol balance: ", (await connection.getBalance(priceReceiverKeypair.publicKey)) / LAMPORTS_PER_SOL);
    });

    it("Airdrop Node One Sale Phase One Tier One", async () => {

        const salePhaseData = await program.account.sogaNodeSalePhaseDetailAccount.fetch(sogaNodeSalePhaseOnePDA.toBase58());

        const tierId: number = 1;
        const tokenIdId: number = 2;

        const [nodeSalePhaseTierPda, nodeSalePhaseTierBump] = getSogaNodeSalePhaseDetailTierAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA);
        console.log("Node Sale Phase one tier two: ", nodeSalePhaseTierPda.toBase58());

        const [nodeSalePhaseTierCollectionPda, nodeSalePhaseTierCollectionBump] = getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, tierId.toString(), sogaNodeSalePhaseOnePDA, nodeSalePhaseTierPda);
        console.log("Node Sale Phase one tier two collection: ", nodeSalePhaseTierCollectionPda.toBase58());

        const nodeSalePhaseTierCollectionMasterPda = getMasterPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier two collection Master: ", nodeSalePhaseTierCollectionMasterPda.toBase58());

        const nodeSalePhaseTierCollectionMetadataPda = getMetadataPda(nodeSalePhaseTierCollectionPda);
        console.log("Node Sale Phase one tier two collection Metadata: ", nodeSalePhaseTierCollectionMetadataPda.toBase58());
        //
        // const nodeSalePhaseTierCollectionTokenAccount = await getAssociatedTokenAddress(nodeSalePhaseTierCollectionPda, nodeSalePhaseTierPda, true, TOKEN_PROGRAM_ID);
        // console.log("Node Sale Phase one tier two collection token account: ", nodeSalePhaseTierCollectionTokenAccount.toBase58());

        const [userDetailPda] = getUserAccountPdaAndBump(program.programId, USER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey);
        console.log("User detail pda: ", userDetailPda.toBase58());

        const [userPhaseTierDetailPda] = getUserTierAccountPdaAndBump(program.programId, USER_TIER_DETAIL_ACCOUNT_PREFIX, phaseOne, sogaNodeSalePhaseOnePDA, userAKeypair.publicKey, userDetailPda, tierId.toString(), nodeSalePhaseTierPda);
        console.log("User tier detail pda: ", userPhaseTierDetailPda.toBase58());

        const [nodeMintAccountPda] = getNodeMintAccount(program.programId, SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
            phaseOne, sogaNodeSalePhaseOnePDA, tierId.toString(), nodeSalePhaseTierPda, nodeSalePhaseTierCollectionPda, tokenIdId.toString());
        console.log("Node Mint Account Pda: ", nodeMintAccountPda.toBase58());

        const nodeMintAccountMasterPda = getMasterPda(nodeMintAccountPda);
        console.log("node mint account master pda: ", nodeMintAccountMasterPda.toBase58());

        const nodeMintAccountMetadataPda = getMetadataPda(nodeMintAccountPda)
        console.log("node mint account metadata pda: ", nodeMintAccountMetadataPda.toBase58());


        const nodeUserTokenAccount = await getAssociatedTokenAddress(nodeMintAccountPda, userAKeypair.publicKey, true, TOKEN_PROGRAM_ID);
        console.log("node user token account: ", nodeUserTokenAccount.toBase58());


        const tx = await program.methods.airdrop(sogaNodeSalePhaseOneBump, nodeSalePhaseTierBump, nodeSalePhaseTierCollectionBump,
            phaseOne, tierId.toString(), tokenIdId.toString())
            .accounts({
                payer: mainSigningAuthorityPubKey,
                signingAuthority: signingAuthorityKeypair.publicKey,
                user: userAKeypair.publicKey,
                salePhaseDetail: sogaNodeSalePhaseOnePDA,
                salePhaseTierDetail: nodeSalePhaseTierPda,
                userDetail: userDetailPda,
                userTierDetail: userPhaseTierDetailPda,
                collectionMintAccount: nodeSalePhaseTierCollectionPda,
                nodeMintAccount: nodeMintAccountPda,
                userTokenAccount: nodeUserTokenAccount,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            }).remainingAccounts([
                {
                    pubkey: nodeSalePhaseTierCollectionMetadataPda,
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: nodeSalePhaseTierCollectionMasterPda,
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: nodeMintAccountMetadataPda,
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: nodeMintAccountMasterPda,
                    isWritable: true,
                    isSigner: false
                }
            ])

            .preInstructions([ComputeBudgetProgram.setComputeUnitLimit({units: 1400_000})])
            .signers([signingAuthorityKeypair])
            .rpc();

        console.log("Your transaction signature", tx);
    });


    it("Remove Events", async () => {
        await delay(2000);

        await program.removeEventListener(initializeSalePhaseEventListener);
        await program.removeEventListener(initializeSalePhaseTierEventListener);
        await program.removeEventListener(updateSalePhaseEventListener);
        await program.removeEventListener(updateSalePhaseTierEventListener);
        // await program.removeEventListener(buyEventListener);
        await program.removeEventListener(airdropEventListener);
    });

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

function getSogaNodeSalePhaseDetailTierAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string, tier: string,
                                                         sogaNodeSalePhaseDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            Buffer.from(tier),
        ],
        programAddress
    )
}

function getSogaNodeSalePhaseDetailTierCollectionAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string, tier: string,
                                                                   sogaNodeSalePhaseDetailPda: PublicKey, sogaNodeSalePhaseTierDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            Buffer.from(tier),
            sogaNodeSalePhaseTierDetailPda.toBuffer()
        ],
        programAddress
    )
}

function getUserAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string,
                                  sogaNodeSalePhaseDetailPda: PublicKey, userAddress: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            userAddress.toBuffer()
        ],
        programAddress
    )
}

function getUserTierAccountPdaAndBump(programAddress: PublicKey, prefix: string, salePhaseName: string,
                                      sogaNodeSalePhaseDetailPda: PublicKey, userAddress: PublicKey, userDetailPdaAddress: PublicKey,
                                      tierId: string, sogaNodeSalePhaseTierDetailPda: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            userAddress.toBuffer(),
            userDetailPdaAddress.toBuffer(),
            Buffer.from(tierId),
            sogaNodeSalePhaseTierDetailPda.toBuffer()
        ],
        programAddress
    )
}

function getNodeMintAccount(programAddress: PublicKey, prefix: string, salePhaseName: string,
                            sogaNodeSalePhaseDetailPda: PublicKey, tierId: string,
                            sogaNodeSalePhaseTierDetailPda: PublicKey, collectionMintAccount: PublicKey, tokenId: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(prefix),
            Buffer.from(salePhaseName),
            sogaNodeSalePhaseDetailPda.toBuffer(),
            Buffer.from(tierId),
            sogaNodeSalePhaseTierDetailPda.toBuffer(),
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