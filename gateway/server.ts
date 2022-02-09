import { ApolloServer } from "apollo-server";
import { ApolloGateway } from "@apollo/gateway";

const gateway = new ApolloGateway({
	serviceList: [
		{ name: "coders", url: "http://localhost:8080/" },
		{ name: "skills", url: "http://localhost:8081/" },
	],
});

function main() {
	const server = new ApolloServer({
		gateway,
	});

	return server.listen().then(({ url }) => {
		console.log(`🚀 Gateway ready at ${url}`);
	});
}

main().catch((error) => {
	console.error("Gateway startup failed:", error);
});
