// convert-svg.ts
import sharp from 'sharp'

// SVG 内容
const svg = `<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50" cy="50" r="45" fill="none" stroke="#2196F3" stroke-width="4" stroke-dasharray="5 3"/>
  <path d="M30,50 Q37,30 50,30 Q63,30 70,50 Q63,70 50,70 Q37,70 30,50
           M70,50 L80,60 L70,70" 
        fill="none" 
        stroke="#2196F3" 
        stroke-width="6"
        stroke-linecap="round"/>
</svg>`

sharp(Buffer.from(svg))
  .resize(1024, 1024)
  .png()
  .toFile('app-icon.png')
  .then(() => console.log('Done!'))
  .catch(err => console.error(err))