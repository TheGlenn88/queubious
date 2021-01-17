function getCookieValue(name) {
    let result = document.cookie.match("(^|[^;]+)\\s*" + name + "\\s*=\\s*([^;]+)")
    return result ? result.pop() : ""
}

setTimeout(() => {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "{{app_url}}/heartbeat");
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON.stringify({token: getCookieValue("queubioustoken")}));
}, 100);