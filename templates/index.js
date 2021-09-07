function toggle_order() {
    let parent = document.querySelector('.plan-list');
    if (parent.dataset.order === undefined) {
        parent.dataset.order = 'default';
    }
    reorder(parent);
}

/**
 * @param {HTMLDivElement} parent
 */
function reorder(parent) {
    let days = Array.from(parent.querySelectorAll('.day'));
    for (let i = 0; i < days.length; i++) {
        let day = days[i];
        parent.removeChild(day);
        day.dataset.number = `${i+1}`;
    }
    if (parent.dataset.order == 'default') {
        parent.dataset.order = 'book';
        book_order(parent, days);
    } else {
        parent.dataset.order = 'default';
        default_order(parent, days);
    }
}
/**
 * @param {HTMLDivElement} parent
 * @param {HTMLDivElement[]} days
 */
function book_order(parent, days) {
    
    while (days.length > 0) {
        let out_r = days.shift();
        let out_l = days.pop();
        parent.appendChild(out_l);
        parent.appendChild(out_r);
        let in_r = days.pop();
        let in_l = days.shift();
        parent.appendChild(in_l);
        parent.appendChild(in_r);
    }
}
/**
 * @param {HTMLDivElement} parent
 * @param {HTMLDivElement[]} days
 */
function default_order(parent, days) {
    days.sort((lhs, rhs) => lhs.dataset.number - rhs.dataset.number);
    for (let day of days) {
        parent.appendChild(day);
    }
}
window.addEventListener('keyup', ev => {
    if (ev.altKey && ev.code === 'KeyO') {
        toggle_order();
    }
});