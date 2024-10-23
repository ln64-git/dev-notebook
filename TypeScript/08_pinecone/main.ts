import { Pinecone } from "npm:@pinecone-database/pinecone";
import { config } from "https://deno.land/x/dotenv@v3.2.2/mod.ts";

// Load environment variables from .env file
const env = config();
const API_KEY = env.PINECONE_API_KEY;
const INDEX_URL = env.PINECONE_INDEX_URL;

// Initialize Pinecone client
const pc = new Pinecone({
  apiKey: API_KEY!,
});

// Function to connect and list indexes
async function connectToPinecone() {
  console.log("Loaded index URL:", INDEX_URL); // Debugging step to ensure the URL is loaded correctly

  // List indexes (verify connection)
  const indexes = await pc.listIndexes();
  console.log("Indexes:", indexes);

  // Use a direct index name (since you know it's 'discord')
  const indexName = "discord";

  // Get index client
  const index = pc.Index(indexName);

  // Example: Query the index
  const queryResponse = await index.query({
    topK: 1,
    vector: [0.1, 0.2, 0.3, 0.4, 0.5], // Example vector (adjust dimensions according to your index)
  });

  console.log("Query response:", queryResponse);
}

connectToPinecone();
