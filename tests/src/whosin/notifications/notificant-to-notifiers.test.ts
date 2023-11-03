import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


test('link a Notificant to a Notifier', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/dcan.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const baseAddress = alice.agentPubKey;
    const targetAddress = alice.agentPubKey;

    // Bob gets the links, should be empty
    let linksOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_notifiers_for_notificant",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Notificant to Notifier
    await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "add_notifier_for_notificant",
      payload: {
        base_notificant: baseAddress,
        target_notifier: targetAddress
      }
    });
    
    await pause(1200);
    
    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_notifiers_for_notificant",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 1);


    await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "remove_notifier_for_notificant",
      payload: {
        base_notificant: baseAddress,
        target_notifier: targetAddress
      }
    });
    
    await pause(1200);

    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_notifiers_for_notificant",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);


  });
});

