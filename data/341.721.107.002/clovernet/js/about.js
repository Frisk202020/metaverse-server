var x = document.getElementById("other");
var x2 = document.getElementById("other2");
var y = document.getElementById('cat');
var z = new Audio("/get/341.721.107.002/clovernet/assets/meow.mp3");

let vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);

function delay(milliseconds){
    return new Promise(resolve => {
        setTimeout(resolve, milliseconds);
    });
}

async function audioPlay() {
    await delay(1000);
    z.play();
}

function boxAppear(){
    x.className = "activeText";
    x2.className = "activeText";
    y.className = "inactiveCat";
    audioPlay();
}

function debug(){
    let a = document.getElementById("us");
    a.innerHTML = `<p>` + vw + `</p>`;
}

function getCss(elemId, property){
    return window.getComputedStyle(document.getElementById(elemId), null).getPropertyValue("left");
}

function correctPosition(currentWidth){
    const RATIO = currentWidth / 1650;
    const CAT = document.getElementById("catBox");
    let left = getCss("catBox", "left");
    console.log(left);   
    CAT.style.left = RATIO*x;
    y = RATIO*y;
}

if (vw > 1650){
    correctPosition(vw);
}