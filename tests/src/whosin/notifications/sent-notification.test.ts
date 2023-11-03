import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createSentNotification, sampleSentNotification } from './common.js';

test('create SentNotification', async () => {
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

    // Alice creates a SentNotification
    const record: Record = await createSentNotification(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read SentNotification', async () => {
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

    const sample = await sampleSentNotification(alice.cells[0]);

    // Alice creates a SentNotification
    const record: Record = await createSentNotification(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created SentNotification
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_sent_notification",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update SentNotification', async () => {
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

    // Alice creates a SentNotification
    const record: Record = await createSentNotification(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the SentNotification
    let contentUpdate: any = await sampleSentNotification(alice.cells[0]);
    let updateInput = {
      original_sent_notification_hash: originalActionHash,
      previous_sent_notification_hash: originalActionHash,
      updated_sent_notification: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "update_sent_notification",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated SentNotification
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_sent_notification",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the SentNotification again
    contentUpdate = await sampleSentNotification(alice.cells[0]);
    updateInput = { 
      original_sent_notification_hash: originalActionHash,
      previous_sent_notification_hash: updatedRecord.signed_action.hashed.hash,
      updated_sent_notification: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "update_sent_notification",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated SentNotification
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_sent_notification",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete SentNotification', async () => {
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

    // Alice creates a SentNotification
    const record: Record = await createSentNotification(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the SentNotification
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "delete_sent_notification",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted SentNotification
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_sent_notification",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
