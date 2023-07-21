import { Container } from 'react-bootstrap';

// Display a cached result
function ResultItem({hashes, requestids, label}){
    let rows = [];
    let sublabels = ['vs Pub_Viral_PDB_Candidate_Set', 'vs Human_PDB_Candidate_Set_v1', 'vs Human_AF4_Candidate_Set'];
    for(let i =0; i < hashes.length; i++)
    {
        rows.push(
            <li>
            <a target="_blank" rel="noopener noreferrer" href={process.env.REACT_APP_BASE_URL + "/result/"+ hashes[i] + "/" + requestids[i] }>
             {sublabels[i]}
            </a>
        </li>
        );
    }

    return(
        <>
            <h3>{label}</h3>
            <ul>
                {rows}
            </ul>
        </>
    )
}

// The content for "Sample results" section
const Results = () => {
      
    return ( 
        <Container className="results">
            <h1>Sample results</h1>

            We provide some sample results on popular human proteins and proteins of known viruses. Each enlisted protein 
            has been compared with three redundant datasets (whole structure comparisons): 
            <ol>
                <li>A dataset that includes virus-associated PDB files (<a href={process.env.REACT_APP_BASE_URL + "/lists/viral_v1.csv"} 
                download>Pub_Viral_PDB_Candidate_Set</a>)</li>
                <li>A dataset that includes human-associated PDB files (<a href={process.env.REACT_APP_BASE_URL + "/lists/human_v1.csv"} 
                download>Human_PDB_Candidate_Set_v1</a>)</li>
                <li>Human proteome predicted by AlphaFold (v4)</li>
            </ol>
            

            <span className='italics-note'>(Links below open in a new tab or window.)</span>
            <br/><br/>

            <ResultItem 
            label={"Epidermal growth factor receptor (EGFR)"} 
            hashes={["2884304533356533217", "14333981677748677289", "13733435421094451075"]} 
            requestids={["70", "71", "69"]} />

            <ResultItem 
            label={"Methylenetetrahydrofolate Reductase (MTHFR)"} 
            hashes={["310988967130001695", "13981873033986751502", "13174928213191213484"]} 
            requestids={["88", "89", "90"]} />

            <ResultItem 
            label={"Tumor Necrosis Factor (TNF)"} 
            hashes={["17508868312800103003", "142155750322977784", "13266090945552367357"]} 
            requestids={["73", "74", "75"]} />

            <ResultItem 
            label={"Tumor Protein P53 (TP53)"} 
            hashes={["2679272980615299704", "14955613678644191222", "8100757110781608523"]} 
            requestids={["76", "77", "78"]} />

            <ResultItem 
            label={"Vascular Endothelial Growth Factor A (VEGFA)"} 
            hashes={["14091359929275865096", "7106532774499802238", "9433262834717873917"]} 
            requestids={["85", "86", "87"]} />
            
            <hr/>


            <ResultItem 
            label={"SARS-CoV-2 Spike protein (Severe acute respiratory syndrome coronavirus 2)"} 
            hashes={["14771271293635504095", "15699423908466110685", "10908241797618001551"]} 
            requestids={["111", "72", "67"]} />

            <ResultItem 
            label={"Ebolavirus nucleoprotein"} 
            hashes={["12114120256680178891", "3352930943076875470", "14476988350547505275"]} 
            requestids={["96", "97", "98"]} />

            <ResultItem 
            label={"EBV Major capsid protein [Epstein-Barr virus (strain B95-8) (HHV-4) (Human herpesvirus 4)]"} 
            hashes={["18213064896821283609", "17423225503856253111", "10908197535272672536"]} 
            requestids={["99", "100", "101"]} />

            <ResultItem 
            label={"HIV-1 capsid protein (p24) (Human immunodeficiency virus 1)"} 
            hashes={["6672365462698496814", "9819962490100943960", "8463941944173890411"]} 
            requestids={["93", "94", "95"]} />

            <ResultItem 
            label={"HPV16 Major capsid protein L1 (Human papillomavirus type 16)"} 
            hashes={["8038131716550248495", "6307513668298468232", "2084168050454501796"]} 
            requestids={["79", "80", "81"]} />

            <ResultItem 
            label={"MPXV E4 (Monkeypox Virus)"} 
            hashes={["18190365787398777031", "5609673495071350271", "6269562845458594711"]} 
            requestids={["82", "83", "84"]} />

        </Container>
    )    
}

export default Results;
