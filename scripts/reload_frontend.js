// Thanks for Sky Leite: https://discord.com/channels/960281551428522045/960284327445418044/1057401742591021077
const plugin = require("../plugin.json");
const CDP = require('chrome-remote-interface');

const options = {
    host: process.env.DECKIP,
    port: 8081,
};

async function main() {
    let client;
    try {
        // connect to endpoint
        client = await CDP({
            ...options,
            target: (targets) => targets.find((target) => target.title == "Steam"),
        });

        // extract domains
        const { Network, Page, Runtime } = client;

        await Runtime.evaluate({ expression: `console.log("Reloading ${plugin.name} from an unbelievably stupid dev script")` });
        await Runtime.evaluate({ expression: `importDeckyPlugin("${plugin.name}")` });
        console.log("Frontend reloaded");
    } catch (err) {
        console.error(err);
    } finally {
        if (client) {
            await client.close();
        }
    }
}

main();