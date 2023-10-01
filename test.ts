import { Flair } from "./bindings/Flair";
import { GetFlairsJson } from "./bindings/GetFlairsJson";
import { AddFlairJson } from "./bindings/AddFlairJson";
import { DeleteFlairJson } from "./bindings/DeleteFlairJson";
import { GetUserFlairJson } from "./bindings/GetUserFlairJson";
import { AddUserFlairJson } from "./bindings/AddUserFlairJson";
import { DeleteUserFlairJson } from "./bindings/DeleteUserFlairJson";

const PORT = 6969;
let success = 0;
let failure = 0;
let tot = 0;

(async () => {
    const community_actor_id = 'http://localhost/c/play';
    const user_actor_id = 'http://localhost/u/Nerd02';

    console.log('Welcome to the "flair" testing script. The script assumes the database to be empty before execution.\nIf the first test fails you might have to start the dev server with "cargo run -- serve".\nIf the 2nd or 3rd tests fail you might have to delete your "flairs.db" file.\n');

    test('server is online', await isServerOnline());
    test('no saved flairs on startup', (await getFlairs({ community_actor_id, mod_only: true })).length === 0);
    test('user doesn\'t have a flair on startup', await getUserFlair({ community_actor_id, user_actor_id }) === null);
    test('add user flair', await addFlair({ community_actor_id, display_name: 'TEMP', mod_only: false, name: 'auth', path: '' }));
    test('flair got added', (await getFlairs({ community_actor_id, mod_only: false })).length === 1);
    test('update existing user flair', await addFlair({ community_actor_id, display_name: 'AuthCenter', mod_only: false, name: 'auth', path: '' }));
    test('existing flair got updated', (await getFlairs({ community_actor_id, mod_only: false }))[0].display_name === 'AuthCenter');
    test('add mod only user flair', await addFlair({ community_actor_id, display_name: 'Based', mod_only: true, name: 'based', path: '' }));
    test('mod flair got added', (await getFlairs({ community_actor_id, mod_only: true })).length === 2);
    test('assign flair to user', await assignUserFlair({ community_actor_id, user_actor_id, flair_name: 'auth' }));
    test('flair got assigned', (await getUserFlair({ community_actor_id, user_actor_id }))?.name === 'auth' ?? false);
    test('remove flair from user', await deleteUserFlair({ community_actor_id, user_actor_id }));
    test('user is now unflaired', await getUserFlair({ community_actor_id, user_actor_id }) === null);
    test('reassign flair to user', await assignUserFlair({ community_actor_id, user_actor_id, flair_name: 'auth' }));
    test('change flair', await assignUserFlair({ community_actor_id, user_actor_id, flair_name: 'based' }));
    test('flair got changed', (await getUserFlair({ community_actor_id, user_actor_id }))?.name === 'based' ?? false);
    test('delete flair while it\'s assigned to user', await deleteFlair({ community_actor_id, name: 'based' }));
    test('flair got removed', (await getFlairs({ community_actor_id, mod_only: true })).length === 1);
    test('user is now unflaired', await getUserFlair({ community_actor_id, user_actor_id }) === null);
    await deleteFlair({ community_actor_id, name: 'auth' });    //Cleanup

    console.log(`\nTests over:\n\t✅ - Passed ${success}/${tot} \n\t❌ - Failed ${failure}/${tot}`);
})();

function test(prompt: string, ok: boolean) {
    console.log(`Test: ${prompt}`);
    tot++;
    if (ok) {
        console.log('┕━━━ ✅ - Passed');
        success++;
    } else {
        console.log('┕━━━ ❌ - Failed')
        failure++;
    }
}

/*  API FUNCTIONS    */

async function isServerOnline() {
    try {
        const res = await fetch(`http://localhost:${PORT}`, { method: 'HEAD' });
        return res.ok;
    } catch (e) {
        return false;
    }
}

async function getFlairs(params: GetFlairsJson) {
    const res = await GET('/v1/community', params);
    return await res.json() as Flair[];
}

async function addFlair(params: AddFlairJson) {
    const res = await PUT('/v1/community', params);
    return res.ok;
}

async function deleteFlair(params: DeleteFlairJson) {
    const res = await DELETE('/v1/community', params);
    return res.ok;
}

async function getUserFlair(params: GetUserFlairJson) {
    const res = await GET('/v1/user', params);
    return await res.json() as Flair | null;
}

async function assignUserFlair(params: AddUserFlairJson) {
    const res = await PUT('/v1/user', params);
    return res.ok;
}

async function deleteUserFlair(params: DeleteUserFlairJson) {
    const res = await DELETE('/v1/user', params);
    return res.ok;
}

/*  HTTP METHOD WRAPPERS    */

async function GET(endpoint: string, params: object) {
    const query = Object.entries(params).map(o => `${o[0]}=${o[1]}`);
    const url = `http://localhost:${PORT}/api${endpoint}?${query.join('&')}`;

    return fetch(url, {
        headers: { "Content-Type": "application/json" }
    });
}

async function PUT(endpoint: string, params: object) {
    const url = `http://localhost:${PORT}/api${endpoint}`;

    return fetch(url, {
        headers: { "Content-Type": "application/json" },
        method: 'PUT',
        body: JSON.stringify(params)
    });
}

async function DELETE(endpoint: string, params: object) {
    const url = `http://localhost:${PORT}/api${endpoint}`;

    return fetch(url, {
        headers: { "Content-Type": "application/json" },
        method: 'DELETE',
        body: JSON.stringify(params)
    });
}
