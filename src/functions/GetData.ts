export async function GetData(url = '', data = {}) {
    const response = await fetch(url, {
        method: 'GET'
    });

    return await response.json();// parses JSON response into native JavaScript objects
}