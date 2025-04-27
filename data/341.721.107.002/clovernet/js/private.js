const DATABASE = {
    "Clover":"FloweyTheFlower",
    "Clarence":"azerty"
}
const BTN = document.getElementById("btn");
const P = document.getElementById("message");

BTN.onclick = function(){
    let username = document.getElementById("user").value;
    let password = document.getElementById("pass").value;
    switch(username){
        case "Clarence":
            if (password === DATABASE["Clarence"]){
                location.href="private/clarence.html";
            }
            else{
                P.style.opacity = 1;
                P.textContent = "Wrong password !";
            }
            break;
        default:
            P.style.opacity = 1;
            P.textContent = "Member not found !";
    }
}