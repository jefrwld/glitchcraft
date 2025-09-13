export function glitch_frame(frame) {
    let mvs = frame["mv"];
    if (!mvs) return;
    
    let fwd_mvs = mvs["forward"];
    if (!fwd_mvs) return;
    
    // Mit Null-Checks!
    for (let i = 0; i < fwd_mvs.length; i++) {
        let row = fwd_mvs[i];
        if (!row) continue;  // Skip wenn row null ist
        
        for (let j = 0; j < row.length; j++) {
            let mv = row[j];
            if (!mv) continue;  // Skip wenn mv null ist
            
            mv[0] = 0;  // Jetzt sicher
        }
    }
}
