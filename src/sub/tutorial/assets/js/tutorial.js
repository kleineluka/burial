/* Annotations for Steps (one will autoplay) */
let annotation_one;
let annotation_two;
let annotation_three;
let annotation_four;
function oneAnnotation() {
    const selected_element = document.getElementById('sidebar-items');
    annotation_one = RoughNotation.annotate(selected_element, { type: 'bracket', color: '#F1E2C5', padding: [-50, 10], strokeWidth: 3, brackets: ['right'] });
    annotation_one.show();
}
setTimeout(function () {
    oneAnnotation();
}, 1000);

function twoAnnotation() {
    const selected_element = document.getElementById('mod-manager');
    annotation_two = RoughNotation.annotate(selected_element, { type: 'circle', color: '#F595B2', padding: [-1, -10], strokeWidth: 3, animationDuration: 2000 });
    annotation_two.show();
}

function threeAnnotation() {
    const selected_element = document.getElementById('resources');
    annotation_three = RoughNotation.annotate(selected_element, { type: 'circle', color: '#F595B2', padding: [-1, -10], strokeWidth: 3, animationDuration: 2000 });
    annotation_three.show();
}

function fourAnnotation() {
    const selected_element = document.getElementById('settings');
    annotation_four = RoughNotation.annotate(selected_element, { type: 'circle', color: '#F595B2', padding: [-1, -10], strokeWidth: 3, animationDuration: 2000 });
    annotation_four.show();
}

/* Handle Navigation Buttons (can't be automatic system as annotations are specific) */
function oneNext() {
    // Remove annotation
    annotation_one.remove();
    // Hide step one, show step two
    const stepOne = document.getElementById('step-one');
    const stepTwo = document.getElementById('step-two');
    stepOne.classList.add('hidden');
    stepTwo.classList.remove('hidden');
    // Annotate step two
    twoAnnotation();
}

function twoBack() {
    // Remove annotation
    annotation_two.remove();
    // Hide step two, show step one
    const stepOne = document.getElementById('step-one');
    const stepTwo = document.getElementById('step-two');
    stepOne.classList.remove('hidden');
    stepTwo.classList.add('hidden');
    // Annotate step one
    oneAnnotation();
}

function twoNext() {
    // Remove annotation
    annotation_two.remove();
    // Hide step two, show step three
    const stepTwo = document.getElementById('step-two');
    const stepThree = document.getElementById('step-three');
    stepTwo.classList.add('hidden');
    stepThree.classList.remove('hidden');
    // Annotate step three
    threeAnnotation();
}

function threeBack() {
    // Remove annotation
    annotation_three.remove();
    // Hide step three, show step two
    const stepTwo = document.getElementById('step-two');
    const stepThree = document.getElementById('step-three');
    stepTwo.classList.remove('hidden');
    stepThree.classList.add('hidden');
    // Annotate step two
    twoAnnotation();
}

function threeNext() {
    // Remove annotation
    annotation_three.remove();
    // Hide step three, show step four
    const stepThree = document.getElementById('step-three');
    const stepFour = document.getElementById('step-four');
    stepThree.classList.add('hidden');
    stepFour.classList.remove('hidden');
    // Annotate step four
    fourAnnotation();
}

function fourBack() {
    // Remove annotation
    annotation_four.remove();
    // Hide step four, show step three
    const stepThree = document.getElementById('step-three');
    const stepFour = document.getElementById('step-four');
    stepThree.classList.remove('hidden');
    stepFour.classList.add('hidden');
    // Annotate step three
    threeAnnotation();
}

// Silly, silly!
document.getElementById('sidebarHome').addEventListener('click', function() {
    Swal.fire({
        title: 'Hey you!',
        text: 'Why don\'t you finish the tutorial first? Please...',
        confirmButtonText: 'Fiiiiine..'
    });
});