
// function([string1, string2],target id,[color1,color2])    
consoleText(['Loading...', 'Loading...', 'Loading...', 'Done!'], 'text',['#FE433C','#3C50B1']);

function consoleText(words, id, colors) {

var extra_css = "display: inline";

if (colors === undefined) colors = ['#fff'];
var visible = true;
var con = document.getElementById('console');
var letterCount = 1;
var x = 1;
var waiting = false;
var target = document.getElementById(id)
let num = 0;
target.setAttribute('style', 'color:' + colors[0] + "; " + extra_css);
window.setInterval(function() {


    if (letterCount === 0 && waiting === false && num != words.length) {
        waiting = true;
        target.innerHTML = words[0].substring(0, letterCount)
        window.setTimeout(function() {
            var usedWord = words.shift();
            words.push(usedWord);
            x = 1;
            target.setAttribute('style', 'color:' + colors[0] + "; " + extra_css)
            letterCount += x;
        waiting = false;
        
        }, 1000)
    } 

    else if (letterCount === words[0].length + 1 && waiting === false) {
        if (num < words.length-1) {
            waiting = true;
            window.setTimeout(function() {
                x = -1;
                letterCount += x;
                waiting = false;
                target.setAttribute('style', 'color:' + colors[0] + "; " + extra_css)
        }, 2010)
    }
        target.setAttribute('style', 'color:' + colors[1] + "; " + extra_css); num++;
    } 
    else if (waiting === false) {
        target.innerHTML = words[0].substring(0, letterCount)
        letterCount += x
    }
    }, 120)

    window.setInterval(function() { // Bar
    if (visible === true) {
        con.className = 'console-bar hidden'
        visible = false;

    } else {
        con.className = 'console-bar'
        visible = true;
    }
    }, 400)
}
