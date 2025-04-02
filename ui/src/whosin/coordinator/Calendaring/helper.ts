export function getLocalISOString(date = new Date()) {
    const offsetMinutes = date.getTimezoneOffset(); // Difference from UTC in minutes
    const offsetHours = Math.abs(Math.floor(offsetMinutes / 60));
    const offsetMins = Math.abs(offsetMinutes % 60);
    const sign = offsetMinutes > 0 ? "-" : "+"; // Reverse sign because getTimezoneOffset() gives negative for UTC+
    
    return date.getFullYear() +
        "-" + String(date.getMonth() + 1).padStart(2, '0') +
        "-" + String(date.getDate()).padStart(2, '0') +
        "T" + String(date.getHours()).padStart(2, '0') +
        ":" + String(date.getMinutes()).padStart(2, '0') +
        ":" + String(date.getSeconds()).padStart(2, '0') +
        sign + String(offsetHours).padStart(2, '0') +
        ":" + String(offsetMins).padStart(2, '0');
}

export function reAddOffset(dateString: Date) {
    const date = new Date(dateString);
    const localDate = new Date();
    const localOffset = localDate.getTimezoneOffset();
    const millis = date.getTime() + (localOffset * 60 * 1000 * 1000);
    return new Date(millis)
}