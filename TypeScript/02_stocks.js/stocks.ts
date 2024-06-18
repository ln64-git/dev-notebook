import Stocks from 'stocks.js';

const apiKey = 'XXXX'; // Replace with your actual API key
const stocks = new Stocks(apiKey);

async function fetchStockData() {
  try {
    const options = {
      symbol: 'TSLA',
      interval: '1min',
      amount: 10
    };
    const result: any = await stocks.timeSeries(options); // Use 'any' type for result
    console.log('Stock data:', result);
  } catch (error) {
    console.error('Error fetching stock data:', error);
  }
}

fetchStockData();
