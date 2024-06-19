import { createClient } from "@alpacahq/typescript-sdk/esm/mod";
import type { AlpacaError } from "../types/error";

const client = createClient({
  key: process.env.ALPACA_API_KEY || "YOUR_API_KEY_ID",
  secret: process.env.ALPACA_SECRET_KEY || "YOUR_API_SECRET_KEY",
});

export async function fetchStockData(symbol: string) {
  try {
    const asset = await client.getAsset({
      symbol_or_asset_id: symbol,
    });
    console.log("asset: ", asset);
    return asset;
  } catch (error: any) {
    const parsedError = JSON.parse(error.message) as AlpacaError;
    console.error("Error fetching stock data:", parsedError.message);
    throw error;
  }
}
