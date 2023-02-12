import "@nomicfoundation/hardhat-chai-matchers";
import "@nomiclabs/hardhat-etherscan";
import "@openzeppelin/hardhat-upgrades";
import dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";

dotenv.config();

const chainIds = {
  hardhat: 31337,
  geth: 15,
  goerli: 5,
  mumbai: 80001,
  fuji: 43113,
};

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      {
        version: "0.8.17",
      },
      {
        version: "0.7.0",
      },
    ],
  },
  networks: {
    hardhat: {
      accounts: {
        mnemonic: process.env.WALLET_MNEMONIC!,
      },
      chainId: chainIds.hardhat,
    },
    geth: {
      url: "http://127.0.0.1:8545/",
      chainId: chainIds.geth,
    },
    goerli: {
      url: process.env.GOERLI_CHAIN_URL!,
      accounts: [process.env.WALLET_SECRET!],
      chainId: chainIds.goerli,
    },
    mumbai: {
      url: process.env.MUMBAI_CHAIN_URL!,
      accounts: [process.env.WALLET_SECRET!],
      chainId: chainIds.mumbai,
    },
    fuji: {
      url: process.env.FUJI_CHAIN_URL!,
      accounts: [process.env.WALLET_SECRET!],
      chainId: chainIds.fuji,
    },
  },
};

export default config;
