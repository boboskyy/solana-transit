import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import * as anchor from '@coral-xyz/anchor';
import * as forge from 'node-forge';
import { BN, Program } from '@coral-xyz/anchor';
import { Transit } from '../target/types/transit';
import { program } from '@coral-xyz/anchor/dist/cjs/native/system';
import { expect } from 'chai';
const chalk = require('chalk');

const IDL = require('../target/idl/transit.json');

const walletA = Keypair.generate();
const courierB = Keypair.generate();
const courierC = Keypair.generate();

const courierBKeys = generateRSAKeyPair();
const courierCKeys = generateRSAKeyPair();

const package_name = `package_${Math.floor(Math.random() * 1000)}`;
const package_public_info = "x";

const couriers = [
  {
    courier: courierB.publicKey,
    deliverySecret: encryptSecretWithPublicKey("genesis_package_0001|secret|B", courierBKeys.publicKey),
    deliveryRewardLamports: new BN(LAMPORTS_PER_SOL),
  },
  {
    courier: courierC.publicKey,
    deliverySecret: encryptSecretWithPublicKey("genesis_package_0001|secret|C", courierCKeys.publicKey),
    deliveryRewardLamports: new BN(2),
  },
];

const CONFIG_PRINT_TX_DATA = false;



let context;
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

let transitProgram = anchor.workspace.Transit as Program<Transit>;

describe('Transit Package Tests', () => {

  let packagePDA;

  before(async () => { await provider });

  it("should create a package", async () => {
    await airdrop(walletA.publicKey, 10 * LAMPORTS_PER_SOL);
    await airdrop(courierB.publicKey, 10 * LAMPORTS_PER_SOL);
    await airdrop(courierC.publicKey, 10 * LAMPORTS_PER_SOL);

    packagePDA = await PublicKey.findProgramAddressSync(
      [Buffer.from(package_name), Buffer.from(package_public_info)],
      transitProgram.programId
    )[0];

    await fundPackageRewardAccount(packagePDA);

    let initializePackageTx = await transitProgram.methods
      .initializePackage(package_name, package_public_info, couriers)
      .accounts({
        package: packagePDA,
        creator: walletA.publicKey,
      })
      .signers([walletA])
      .rpc();

    logTestResult("Package creation", initializePackageTx, packagePDA);
  });

  it("first courier should accept the package", async () => {
    let pickupTx = await transitProgram.methods.confirmPickup(package_name, package_public_info)
      .accounts({
        package: packagePDA,
        courier: courierB.publicKey,
        receiver: courierB.publicKey,
      })
      .signers([courierB])
      .rpc();

    let packageAccountData = await transitProgram.account.package.fetch(packagePDA);

    logTestResult("First courier accepts package", pickupTx, packageAccountData);

    console.log(chalk.blue('Couriers B encrypted secret: ', packageAccountData.couriers[0].deliverySecret));
    console.log(chalk.blue('Courier B decrypted secret: ', decryptSecretWithPrivateKey(packageAccountData.couriers[0].deliverySecret, courierBKeys.privateKey)));

    expect(packageAccountData.currentHolder.toBase58()).to.equal(courierB.publicKey.toBase58());
    expect(packageAccountData.confirmations[0].courier.toBase58()).to.equal(courierB.publicKey.toBase58());
    expect(packageAccountData.confirmations[0].receivedAt.toNumber()).to.be.greaterThan(0);
  });

  it("first courier should deliver the package", async () => {
    let deliveryTx = await transitProgram.methods.confirmDelivery(package_name, package_public_info)
      .accounts({
        package: packagePDA,
        courier: courierB.publicKey
      })
      .signers([courierB])
      .rpc();

    let packageAccountData = await transitProgram.account.package.fetch(packagePDA);

    logTestResult("First courier delivers package", deliveryTx, packageAccountData);

    expect(packageAccountData.currentHolder.toBase58()).to.equal(courierB.publicKey.toBase58());
    expect(packageAccountData.confirmations[0].deliveredAt.toNumber()).to.be.greaterThan(0);
    expect(packageAccountData.confirmations[0].deliveredAt.toNumber()).to.be.greaterThanOrEqual(packageAccountData.confirmations[0].receivedAt.toNumber());
  });

  it("second courier should accept the package", async () => {

    let courierBLamportsBefore = await provider.connection.getBalance(courierB.publicKey);

    let pickupTx = await transitProgram.methods.confirmPickup(package_name, package_public_info)
      .accounts({
        package: packagePDA,
        courier: courierC.publicKey,
        receiver: courierB.publicKey,
      })
      .signers([courierC])
      .rpc();

    let packageAccountData = await transitProgram.account.package.fetch(packagePDA);

    logTestResult("Second courier accepts package", pickupTx, packageAccountData);

    console.log(chalk.blue('Couriers C encrypted secret: ', packageAccountData.couriers[1].deliverySecret));
    console.log(chalk.blue('Courier C decrypted secret: ', decryptSecretWithPrivateKey(packageAccountData.couriers[1].deliverySecret, courierCKeys.privateKey)));

    expect(packageAccountData.currentHolder.toBase58()).to.equal(courierC.publicKey.toBase58());

    let courierBLamportsAfter = await provider.connection.getBalance(courierB.publicKey);
    expect(courierBLamportsAfter).to.be.greaterThan(courierBLamportsBefore);

    console.log(chalk.green(`Courier B's balance increased by: ${courierBLamportsAfter - courierBLamportsBefore} lamports`));
  });

  it("second courier should deliver the package", async () => {
    let secondCourierLamportsBefore = await provider.connection.getBalance(courierC.publicKey);

    let deliveryTx = await transitProgram.methods.confirmDelivery(package_name, package_public_info)
      .accounts({
        package: packagePDA,
        courier: courierC.publicKey
      })
      .signers([courierC])
      .rpc();

    let packageAccountData = await transitProgram.account.package.fetch(packagePDA);

    logTestResult("Second courier delivers package", deliveryTx, packageAccountData);

    expect(packageAccountData.currentHolder.toBase58()).to.equal(courierC.publicKey.toBase58());
    expect(packageAccountData.confirmations[1].deliveredAt.toNumber()).to.be.greaterThan(0);
    expect(packageAccountData.confirmations[1].deliveredAt.toNumber()).to.be.greaterThanOrEqual(packageAccountData.confirmations[1].receivedAt.toNumber());

    let secondCourierLamportsAfter = await provider.connection.getBalance(courierC.publicKey);
    expect(secondCourierLamportsAfter).to.be.greaterThan(secondCourierLamportsBefore);

    console.log(chalk.green(`Courier C's balance increased by: ${secondCourierLamportsAfter - secondCourierLamportsBefore} lamports`));
  });

});


async function airdrop(wallet: PublicKey, lamports: number) {
  const airdropSig = await provider.connection.requestAirdrop(wallet, lamports);
  await provider.connection.confirmTransaction(airdropSig);
  console.log(chalk.blue(`Airdropped ${lamports} lamports to ${wallet.toBase58()}`));
}

async function fundPackageRewardAccount(packagePDA: PublicKey) {
  const fundRewardAccountTx = new anchor.web3.Transaction().add(
    SystemProgram.transfer({
      fromPubkey: walletA.publicKey,
      toPubkey: packagePDA,
      lamports: 3 * LAMPORTS_PER_SOL,
    })
  );
  await provider.sendAndConfirm(fundRewardAccountTx, [walletA]);
  console.log(chalk.blue(`Funded package reward account with 3 SOL`));
}

function logTestResult(testName: string, txHash: string, data: any) {
  console.log(chalk.bold.cyan(`\n--- ${testName} ---`));
  console.log(chalk.green(`Transaction Hash: ${txHash}`));
  if (CONFIG_PRINT_TX_DATA) {
    console.log(chalk.green(`Package Account Data: `));
    console.log(JSON.stringify(data, null, 2));
  }
  console.log(chalk.green('----------------------------\n'));
}

function generateRSAKeyPair() {
  const keypair = forge.pki.rsa.generateKeyPair(2048);
  return {
    publicKey: forge.pki.publicKeyToPem(keypair.publicKey),
    privateKey: forge.pki.privateKeyToPem(keypair.privateKey),
  };
}

function encryptSecretWithPublicKey(secret: string, publicKey: string) {
  const pubKey = forge.pki.publicKeyFromPem(publicKey);
  const encrypted = pubKey.encrypt(secret);
  return forge.util.encode64(encrypted);
}

function decryptSecretWithPrivateKey(encryptedSecret: string, privateKey: string) {
  const privKey = forge.pki.privateKeyFromPem(privateKey);
  const decrypted = privKey.decrypt(forge.util.decode64(encryptedSecret));
  return decrypted;
}
