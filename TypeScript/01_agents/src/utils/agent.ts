import { createOpenAIFunctionsAgent } from "langchain/agents";
import { ChatPromptTemplate } from "@langchain/core/prompts";
// import { pull } from "langchain/hub"; // Comment out since it is causing the error
import { OllamaFunctions } from "@langchain/community/experimental/chat_models/ollama_functions";

// Mock ChatPromptTemplate with required input variables including URL
const mockPromptTemplate = ChatPromptTemplate.fromTemplate(`
  User: {input}
  URL: {url}
  Agent: {response}
  Scratchpad: {agent_scratchpad}
`);

export async function createAgent() {
  const baseUrl = "http://127.0.0.1:11434";
  const functionModel = new OllamaFunctions({ temperature: 0.1, model: "llama3", baseUrl });

  // Use mock prompt template instead of pulling from the repository
  const prompt = mockPromptTemplate;

  const agent = await createOpenAIFunctionsAgent({
    llm: functionModel,
    tools: [],
    prompt
  });
  return agent;
}
