export async function FetchJson(url = '', data = {}) {
    const response = await fetch(url, {
        method: 'GET'
    });

    return await response.json();// parses JSON response into native JavaScript objects
}


export async function FetchDataPromise(url = '', data = {}) {
    const response = await fetch(url, {
        method: 'GET'
    });

    return await response.json();// parses JSON response into native JavaScript objects
}