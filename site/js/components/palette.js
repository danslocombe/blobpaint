
const palettes = [
    {
        name: "Pico 8",
        data: [
            "#ff77a8",
            "#83769c",
            "#ffccaa",
            "#000000",
        ]
    },
    {
        name: "Old Desert",
        data: [
            "#FFFF88",
            "#FFAA88",
            "#AA8844",
            "#000000",
        ]
    },
    {
        name: "Garden Snail",
        data: [
            "#abbc60",
            "#6f7ba3",
            "#704b30",
            "#211e19",
        ]
    },
    {
        name: "Gaudy Gaudi",
        data: [
            "#F032BC",
            "#864BE7",
            "#F9F871",
            "#000000",
        ]
    },
    {
        name: "Geg",
        data: [
            "#766c91",
            "#813b3d",
            "#f9c885",
            "#523b58",
        ]
    },
    {
        name: "Noir",
        data: [
            "#5f3e53",
            "#ca9d53",
            "#2b3541",
            "#8c7f78",
        ]
    },
    {
        name: "Monochrome",
        data: [
            "#F4F4F4",
            "#9F9f9f",
            "#444444",
            "#0A0A0A",
        ]
    },
    {
        name: "Giraffe",
        data: [
            "#eecd96",
            "#a34e01",
            "#78b3ea",
            "#000000",
        ]
    },
    {
        name: "Tiger",
        data: [
            "#ffa000",
            "#ebd91a",
            "#fef1eb",
            "#000000",
        ]
    },
]


//let current = Math.floor(Math.random() * (palettes.length - 1));
let current = 0;

export function GetPalette() {
    return palettes[current].data;
}

export function GetPaletteName() {
    return palettes[current].name;
}

export function NextPalette() {
    current = (current + 1) % palettes.length;
}

export function PrevPalette() {
    current = (current - 1);
    if (current < 0) {
        current = palettes.length - 1;
    }
}
export function GetPaletteForGif() {
    // Export to format expecte by gif.js
    // Array of triples [r0, g0, b0, r1, g1, b1, ...]
    // Eg
    //[0xFF, 0xFF, 0x88, 0xFF, 0xAA, 0x88, 0xAA, 0x88, 0x44, 0x00, 0x00, 0x00]
    let cols = [];
    let p = palettes[current];
    for (let i = 0; i < 4; i++) {
        const hexstring = p.data[i];

        // bit hacky
        const red = parseInt("0x" + hexstring.substring(1, 3));
        const green = parseInt("0x" + hexstring.substring(3, 5));
        const blue = parseInt("0x" + hexstring.substring(5, 7));
        cols.push(red);
        cols.push(green);
        cols.push(blue);
    }

    return cols;
}