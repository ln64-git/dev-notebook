import { fetchStockData } from "../function/fetchStockData";
import type { AlpacaError } from "../types/error";

export async function runWorkflow(symbol: string) {
  try {
    const stockMetrics = await fetchStockData(symbol);
    console.log("stockMetrics: ", stockMetrics);
    return stockMetrics;
  } catch (error: any) {
    const parsedError = JSON.parse(error.message) as AlpacaError;
    console.error("Error running workflow:", parsedError.message);
    throw error;
  }
}
