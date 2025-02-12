import React, { useEffect, useState } from "react";
import { DappProvider, useDapp } from "@multiversx/sdk-dapp";
import { ProxyNetworkProvider } from "@multiversx/sdk-network-providers";
import { sendTransaction } from "@multiversx/sdk-dapp/services";
import { Address, ContractFunction, TransactionPayload, BigUIntValue } from "@multiversx/sdk-core";
import { Button, Card, CardContent, Input } from "@/components/ui";

const contractAddress = "YOUR_CONTRACT_ADDRESS_HERE";
const provider = new ProxyNetworkProvider("https://devnet-gateway.multiversx.com");

export default function VotingApp() {
  const { account, address, login, logout } = useDapp();
  const [votingId, setVotingId] = useState("");
  const [question, setQuestion] = useState("");
  const [yesVotes, setYesVotes] = useState(0);
  const [noVotes, setNoVotes] = useState(0);

  const fetchQuestion = async () => {
    if (!votingId) return;
    const response = await provider.queryContract({
      contract: new Address(contractAddress),
      func: new ContractFunction("get_question"),
      args: [new BigUIntValue(votingId)],
    });
    setQuestion(response.returnData[0]);
  };

  const fetchResults = async () => {
    if (!votingId) return;
    const response = await provider.queryContract({
      contract: new Address(contractAddress),
      func: new ContractFunction("get_results"),
      args: [new BigUIntValue(votingId)],
    });
    setYesVotes(parseInt(response.returnData[0]));
    setNoVotes(parseInt(response.returnData[1]));
  };

  const vote = async (choice) => {
    if (!votingId || !address) return;
    await sendTransaction({
      sender: address,
      receiver: contractAddress,
      data: new TransactionPayload(`vote@${votingId}@${choice}`),
      gasLimit: 5000000,
    });
    fetchResults();
  };

  return (
    <DappProvider>
      <Card>
        <CardContent>
          {!address ? (
            <Button onClick={login}>Login</Button>
          ) : (
            <Button onClick={logout}>Logout</Button>
          )}
          <Input placeholder="ID de votació" value={votingId} onChange={(e) => setVotingId(e.target.value)} />
          <Button onClick={fetchQuestion}>Obtenir Pregunta</Button>
          <p>{question}</p>
          <Button onClick={() => vote("Sí")}>Votar Sí</Button>
          <Button onClick={() => vote("No")}>Votar No</Button>
          <Button onClick={fetchResults}>Obtenir Resultats</Button>
          <p>Vots Sí: {yesVotes}</p>
          <p>Vots No: {noVotes}</p>
        </CardContent>
      </Card>
    </DappProvider>
  );
}
