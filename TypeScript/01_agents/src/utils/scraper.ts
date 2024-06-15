// utils/scraper.ts
import { CheerioWebBaseLoader } from "@langchain/community/document_loaders/web/cheerio";
import { TavilySearchResults } from "@langchain/community/tools/tavily_search";
import { OpenAIEmbeddings } from "@langchain/openai";
import { RecursiveCharacterTextSplitter } from "langchain/text_splitter";
import { MemoryVectorStore } from "langchain/vectorstores/memory";

export const searchTool = new TavilySearchResults();

const loader = new CheerioWebBaseLoader("https://google.com");

async function createRetriever() {
  const rawDocs = await loader.load();
  const splitter = new RecursiveCharacterTextSplitter({
    chunkSize: 1000,
    chunkOverlap: 200
  });
  const docs = await splitter.splitDocuments(rawDocs);
  const vectorstore = await MemoryVectorStore.fromDocuments(docs, new OpenAIEmbeddings());
  return vectorstore.asRetriever();
}
