const OFFSET = 100;
const DURATION = 500;
const map = new WeakMap();

const ob = new IntersectionObserver((entries) => {
    for (const entry of entries) {
        if (entry.isIntersecting) {
            const animation = map.get(entry.target)
            if (animation) {
                animation.play();
                ob.unobserve(entry.target); // 停止观察
            }
        }
    }
});

function belowViewport(el) {
    const rect = el.getBoundingClientRect();
    return rect.top > window.innerHeight;
}

export default {
    mounted(el) {
        if(!belowViewport(el)){
            return;
        }
        // 上划动画
        const animation = el.animate([
            { transform: `translateY(${OFFSET}px)`, opacity: 0.5 },
            { transform: 'translateY(0)', opacity: 1 }
        ],
            {
                duration: DURATION,
                easing: 'ease-out',
                fill: 'forwards'
            });

        animation.pause();
        ob.observe(el);
        map.set(el, animation);
    },
    unmounted(el) {
        ob.unobserve(el);
    }
}