const fs = require('fs');

// ASCII Art for IzacOS
const asciiArt = fs.readFileSync('/etc/motd', 'utf8');

const messages = [
    "The machine obeys, eventually.",
    "One more restart ought to do it.",
    "Today feels like a good day to ship something small.",
    "The logs know. The logs always know.",
    "A calm mind and a working build.",
    "May your grep be swift and your bugs be obvious.",
    "You are only one weird fix away from success.",
    "Another beautiful day to overengineer a tiny problem.",
    "If it compiles, that is a form of affection.",
    "The codebase remembers everything.",
    "Build first. Regret later.",
    "Somewhere, a cron job is believing in you.",
    "It worked once, which means it can work again.",
    "Small steps. Fewer fires.",
    "The terminal has spoken.",
    "May your side project outlive your attention span.",
    "A fresh shell, a fresh start.",
    "Even bad code can become lore.",
    "The bug is temporary. The commit is forever.",
    "Today's vibe: suspiciously productive."
];

// Function to get a random message
function getRandomMessage() {
    return messages[Math.floor(Math.random() * messages.length)];
}

// Output the MOTD
console.log(asciiArt);
console.log(getRandomMessage());
console.log(); // Add a newline for spacing