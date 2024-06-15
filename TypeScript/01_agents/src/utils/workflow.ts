// main.ts
import { createAgent } from "./agent";
import { AgentExecutor } from "langchain/agents";

export async function runWorkflow() {
  try {
    const agent = await createAgent();

    // Example usage of AgentExecutor
    const agentExecutor = new AgentExecutor({
      agent,
      tools: []
    });

    // Example input
    const result = await agentExecutor.invoke({
      input: "analyze the following website and compress into 3-5 sentences",
      url: "https://en.wikipedia.org/wiki/Solana_(blockchain_platform)",
      response: "",
      agent_scratchpad: ""
    });

    console.log(result);
  } catch (error) {
    console.log(error);
  }
}

