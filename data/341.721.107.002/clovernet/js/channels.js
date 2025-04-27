const GENERAL = document.getElementById("general");
const CLOVER = document.getElementById("CloverInbox");
const TEAM = document.getElementById("TeamInbox")
const BOX = document.getElementById("container");
const AUTHORIZED = [
    "general", "Clover"
];

const INDICATORS = {
    "general":document.getElementById("generalIndicator"),
    "Clover":document.getElementById("CloverIndicator"),
    "Team":document.getElementById("TeamIndicator")
};

function clearInactive(currentIndicator){
    for (var key in INDICATORS){
        const element = INDICATORS[key];
        if (key !== currentIndicator){
            element.innerHTML = `+`;
        }
    }
}

function setChannelPermission(currentChannel){
    const INPUT = document.getElementById("sendMessage");
    if (AUTHORIZED.includes(currentChannel)){
        INPUT.placeholder = "message";
        INPUT.classList.remove("forbid");
        INPUT.disabled = false;
    }
    else{
        INPUT.placeholder = "Can't send messages in this channel !";
        INPUT.classList.add("forbid");
        INPUT.disabled = true;
    }
}

GENERAL.onclick = function(){
    const INDICATOR = document.getElementById("generalIndicator");
    clearInactive("general");
    setChannelPermission("general");
    generateMessageLog(GENERAL_DATABASE);

    if (INDICATOR.innerHTML === `+`){
        INDICATOR.innerHTML = `-`;
        BOX.className = "active";

    }
    else{
        INDICATOR.innerHTML = `+`;
        BOX.className = "inactive";
    }
}

CLOVER.onclick = function(){
    const INDICATOR = document.getElementById("CloverIndicator");
    clearInactive("Clover");
    setChannelPermission("Clover");
    generateMessageLog(CLOVER_INBOX_DATABASE);

    if (INDICATOR.innerHTML === `+`){
        INDICATOR.innerHTML = `-`;
        BOX.className= "active";
    }
    else{
        INDICATOR.innerHTML = `+`;
        BOX.className = "inactive";
    }  
}

TEAM.onclick = function(){
    const INDICATOR = document.getElementById("TeamIndicator");
    clearInactive("Team");
    setChannelPermission("Team");
    generateMessageLog(TEAM_INBOX_DATABASE);

    if (INDICATOR.innerHTML === `+`){
        INDICATOR.innerHTML = `-`;
        BOX.className= "active";
    }
    else{
        INDICATOR.innerHTML = `+`;
        BOX.className = "inactive";
    }  
}

