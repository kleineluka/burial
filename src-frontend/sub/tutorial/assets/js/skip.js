/* Delayed Annotation (animate.css breaks it */
function delayedAnnotation() {
    const title = document.getElementById('button-tutorial');
    const annotation = RoughNotation.annotate(title, { type: 'bracket', color: '#F1E2C5', padding: [ 6, 2 ], strokeWidth: 3, brackets: ['bottom'] });
    annotation.show();
    // get the svg element
    const svg = document.querySelector('svg');
    // get the second path element
    const path = svg.querySelector('path:nth-child(2)');
    // create a new div then add handwriting to it
    setTimeout(function () {
        const handwriting = document.createElement('div');
        handwriting.id = 'annotation-handwriting';
        handwriting.style.position = 'absolute';
        handwriting.style.top = path.getBoundingClientRect().top + 'px';
        handwriting.style.left = path.getBoundingClientRect().left + 'px';
        handwriting.style.left = (parseInt(handwriting.style.left, 10) - 20) + 'px';
        handwriting.style.top = (parseInt(handwriting.style.top, 10) + 10) + 'px';
        // increase width
        handwriting.style.zIndex = '9999';
        document.body.appendChild(handwriting);
        new Vara("#annotation-handwriting", "../../../assets/ext/vara.json", [{
            text: "you probably should.."
        }], {
            fontSize: 30,
            color: "#F1E2C5",
            textAlign: "center",
            strokeWidth: 1.5,
            duration: 1000
        });
    }, 1000);
}
setTimeout(function () {
    delayedAnnotation();
}, 2000);

// update the position of p on window resize
window.addEventListener('resize', function () {
    // delay 500 ms
    setTimeout(function () {
        const svg = document.querySelector('svg');
        // get the second path element
        const path = svg.querySelector('path:nth-child(2)');
        const handwriting = document.getElementById('annotation-handwriting');
        handwriting.style.top = path.getBoundingClientRect().top + 'px';
        handwriting.style.left = path.getBoundingClientRect().left + 'px';
    }, 500);
});