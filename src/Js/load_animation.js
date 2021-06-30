
// function([string1, string2],target id,[color1,color2])    
consoleText(['Loading...', 'Please wait...'], 'text',['#adadad']);

function consoleText(words, id, colors) {
  if (colors === undefined) colors = ['#fff'];
  var visible = true;
  var con = document.getElementById('console');
  var letterCount = 1;
  var x = 1;
  var waiting = false;
  var target = document.getElementById(id)
  target.setAttribute('style', 'color:' + colors[0])
  window.setInterval(function() {

    if (letterCount === 0 && waiting === false) {
      waiting = true;
      target.innerHTML = words[0].substring(0, letterCount)
      window.setTimeout(function() {
        var usedColor = colors.shift();
        colors.push(usedColor);
        var usedWord = words.shift();
        words.push(usedWord);
        x = 1;
        target.setAttribute('style', 'color:' + colors[0])
        letterCount += x;
        waiting = false;
        
      }, 1000)

    } 
    
    else if (letterCount === words[0].length + 1 && waiting === false) {
      waiting = true;
      window.setTimeout(function() {
        target.setAttribute('style', 'color:' + colors[0])
        x = -1;
        letterCount += x;
        waiting = false;
      }, 1000); target.setAttribute('style', 'color:' + colors[0])
    } 
    
    else if (waiting === false) {
      target.innerHTML = words[0].substring(0, letterCount)
      target.setAttribute('style', 'color:' + colors[0])
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
