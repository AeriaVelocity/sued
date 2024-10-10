/* 
 * vi Navigation - Scroll through a web page using keys inspired by the vi editor
 * Author: Arsalan 'Aeri' Kazmi <sonicspeed848@gmail.com>
 * Date: 2024-10-10
 * Licence: ISC
 * 
 * This file was made for the sued documentation.
 * See https://aeriavelocity.github.io/sued for more information.
 * You are welcome to redistribute and modify this script under the terms of the
 * ISC licence - see https://opensource.org/licenses/ISC.
 */

document.addEventListener('keydown', function(e) {
    switch (e.key) {
        /* Basic movement */
        case 'h':
            window.scrollBy(-50, 0);
            break;
        case 'j':
            window.scrollBy(0, 50);
            break;
        case 'k':
            window.scrollBy(0, -50);
            break;
        case 'l':
            window.scrollBy(50, 0);
            break;
        
        /* Page movement */
        case 'u':
            window.scrollBy(0, -window.innerHeight);
            break;
        case 'd':
            window.scrollBy(0, window.innerHeight);
            break;

        /* Document movement */
        case 'g':
            window.scrollTo(0, 0);
            break;
        case 'G':
            window.scrollTo(0, document.body.scrollHeight);
            break;
    }
});