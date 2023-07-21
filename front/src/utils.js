
// Filtering structure IDs in user input
export function extractStructureID (structureId) {
    const regex = /[A-Z|0-9|a-z]{4}/g;
    const afRegex = /AF-[A-Z|0-9|a-z]{3,}-F[0-9]+-model_v4/g;
    const esmRegex = /MGYP[0-9]{12}/g;
    let matched = structureId.match(afRegex);
    let result = "";
    
    if(matched  === null)
    {
        matched = structureId.match(esmRegex);
    }

    if(matched  === null)
    {
        matched = structureId.match(regex);
    }

    if(matched !== null)
    { 
        if(matched.length > 0){ 
            matched = matched[0];
            if(matched.length > 3){ 
                result = matched;
            }
        }
    }   

    if(result !== structureId)
    {
        result = "";
    }
    return result;
}

// Filtering structure chain IDs in user input
export function extractCompositeID(structureId) {
    const regex = /[A-Z|0-9|a-z]{4}_[A-Z|0-9|a-z]/g;
    const afRegex = /AF-[A-Z|0-9|a-z]{3,}-F[0-9]+-model_v4_[A-Z|0-9|a-z]/g;
    const esmRegex = /MGYP[0-9]{12}_[A-Z|0-9|a-z]/g;
    let parts = structureId.match(afRegex);
    let result = "";
    
    if(parts  === null)
    {
        parts = structureId.match(esmRegex);
    }

    if(parts  === null)
    {
        parts = structureId.match(regex);
    }

    if(parts !== null)
    {
        if(parts.length > 0) {
            parts = parts[0].split('_');
        }

        if(parts.length > 1 && parts[0].length > 3){ 
            result = parts.join('_');
        }
    }   

    if(result !== structureId)
    {
        result = "";
    }
    return result;
}