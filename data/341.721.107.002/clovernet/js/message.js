const MESSAGE = document.getElementById("logBox");
const HOST = "Clarence";
const CURRENT_HOUR = new Date().getHours();
const CURRENT_MIN = new Date().getMinutes();
let CURRENT_DATABASE;
const GENERAL_DATABASE = [
    "user=Clover;message=How Are you Clarence?;date=15/07/42",
    "user=Clarence;message=I'm a bit exhausted with the mission preparation !;date=" + engDateWithParam(CURRENT_HOUR - 4, CURRENT_MIN),
    "user=Nikko;message=Yeah this mission is a lot of pressure chief !;date="+ engDateWithParam(CURRENT_HOUR - 2, CURRENT_MIN),
    "user=Clover;message=I know but keep in mind that this mission is probably the most important one the department has ever seen.;date=" + engDateWithParam(CURRENT_HOUR - 1, CURRENT_MIN)
]
const CLOVER_INBOX_DATABASE = [
    "user=Nikko;message=Hi Clover !;date=12/06/42"
]
const TEAM_INBOX_DATABASE = [
    "user=Clover;message=Case : UNDERDOGS Worm<br><br>From : The President<br><br><i>Hi Clover, thanks for accepting this highly important case for our future. As you know, we're now sure that UNDERDOGS have a worm almost ready to be sent on our HQ. This operation is very confidential in an effort to keep the population at peace. As so, we would want you to remind your spying team that any leak of your activities on this case will be punished.<br><br> Take care, The President</i>;date=15/07/42",
]

const ROLES={
    "Clover":`<span class="chief">`,
    "Clarence":`<span class="veteran">`,
    "Nikko":`<span class="rookie">`
}

function retrieveData(database){
    const DATA = [];
    for (i = 0; i < database.length; i++){
        const RETRIEVE = database[i].split(';').reverse();
        const DATA_ELEM = {};
        for (j = 0; j < RETRIEVE.length; j++){
            const PAIR = RETRIEVE[j].split('=');
            if (PAIR[0] !== ""){
                DATA_ELEM[PAIR[0]] = PAIR[1];
            }
        }
        DATA.push(DATA_ELEM);
    }
    return DATA.reverse();
}

function generateHTML(user, message, date){
    let HTML;
    if (user === HOST){
        HTML = `<div id="host"><div class=message>
                    <p>from: `;
    }
    else{
        HTML = `<div id="other"><div class=message>
                        <p>from: `;
    }
    HTML = HTML + ROLES[user] + user + `</span></p>`;
    HTML = HTML + `<p>` + message + `</p>`;
    HTML = HTML + `<span style="display:flex;justify-content:right"><p class="date">` + date + '</p></span></div></div>'

    return HTML;
}


function generateMessageLog(database){
    const DATA = retrieveData(database);
    CURRENT_DATABASE = database;
    let HTML = ``;
    for (i = 0; i < DATA.length; i++){
        HTML += generateHTML(DATA[i]["user"], DATA[i]["message"], DATA[i]["date"]);
    }
    MESSAGE.innerHTML = HTML;
}

function hover(){
    const IMG = document.getElementById("sendIcon");
    IMG.src = "../../assets/sendIconHover.png";
}
function unhover(){
    const IMG = document.getElementById("sendIcon");
    IMG.src = "../../assets/sendIcon.png";
}
function engDate(){
    const date = new Date();
    const hour = date.getHours();
    const min = date.getMinutes();

    let minStr
    if (min < 10){
        minStr = "0"+min;
    }
    else{
        minStr = min;
    }

    if (hour > 1 && hour < 13){
        return hour + ":" + minStr + " AM";
    }
    else if (hour < 24){
        return hour - 12 + ":" + minStr + " PM";
    }
    else{
        return "0:" + minStr + " PM";
    }
}
function engDateWithParam(hour, min){
    let minStr
    if (min < 10){
        minStr = "0"+min;
    }
    else{
        minStr = min;
    }

    if (hour > 1 && hour < 13){
        return hour + ":" + minStr + " AM";
    }
    else if (hour < 24){
        return hour - 12 + ":" + minStr + " PM";
    }
    else{
        return "0:" + minStr + " PM";
    }
}
function sendMessage(){
    const SEND = document.getElementById("sendMessage");
    if (SEND.value !== ""){
        CURRENT_DATABASE.push("user=" + HOST + ";message=" + SEND.value + ";date=" + engDate() + ";");
        generateMessageLog(CURRENT_DATABASE);
        SEND.value = "";
    }
}

const SEND_BTN = document.getElementById("sendIcon");
SEND_BTN.onclick = sendMessage;