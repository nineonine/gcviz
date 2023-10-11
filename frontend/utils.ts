const darkenColor = (hex: string, percent: number): string => {
    // Convert the hex to RGB values
    let r: number = parseInt(hex.substring(1, 3), 16);
    let g: number = parseInt(hex.substring(3, 5), 16);
    let b: number = parseInt(hex.substring(5, 7), 16);

    // Calculate the adjustment value
    let adjust = (amount: number, color: number) => {
        return Math.round(color * (1 - amount));
    };

    r = adjust(percent, r);
    g = adjust(percent, g);
    b = adjust(percent, b);

    // Convert the RGB values back to hex
    return "#" + r.toString(16).padStart(2, '0') + g.toString(16).padStart(2, '0') + b.toString(16).padStart(2, '0');
}
