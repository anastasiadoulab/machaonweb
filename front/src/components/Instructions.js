import { Container } from 'react-bootstrap';
import { Link } from "react-router-dom";

// Text for 'Instructions' section
const Instructions = () => { 
    
    return ( 
        <Container className="instructions">
            <h1>Instructions</h1>

            <h3>Quick Start</h3>

            <ol>
                <li>
                    Pick a structure from the following sources: <a href="https://www.rcsb.org">RCSB PDB</a>, <a 
                    href="https://alphafold.ebi.ac.uk">AlphaFold DB</a>, <a href="https://esmatlas.com">ESM Metagenomic Atlas
                    </a> (MGnify proteins only)
                </li>

                <li>
                    Input the PDB ID, or full AlphaFold filename or MGnify ID along with Chain ID (seperated by underscore) as a reference protein.
                </li>

                <li>Select a predefined list of proteins or enter your own list.</li>

                <li>Select a mode of comparison (whole structure, domain, segment). For segment mode, specify a range of residue positions.</li>

                <li>You can choose to enable meta-analysis. If it is enabled, you can also choose a Gene Ontology term for searching relevant proteins in the results.</li>

                <li>If all inputs are valid (no red exclamation marks appearing on the right of the fields), the submission button will become clickable. You can submit your request.</li>

                <li>After submission, you should <b>keep</b> the link in order to access the results whenever they are ready.</li>

                <li>In case you miss the link, you can make a new request some hours later (to ensure that the job has finished, if it was successful) and you can access the results in a short time.</li>

                <li>Most of the files in the results can be processed with a spreadsheet software. The files in the candidates folder contain the results (the longer the filename, the more information).</li>
            </ol>


            <h3>Detailed Instructions</h3>
                
            Machaon allows you to look for similar structures (candidates) in a search space of your preference and 
            analyze these structural relationships beyond structure. Please review these brief instructions below for your convenience:
            
            <ul>
                
                <li> At first, you need to specify a reference identifier (<a href="https://www.rcsb.org">RCSB PDB</a> ID or <a 
                    href="https://alphafold.ebi.ac.uk">AlphaFold DB</a> v4 full filename or MGnify protein ID 
                    that starts with MGYP from <a href="https://esmatlas.com">ESM Metagenomic Atlas</a>) and the specific chain identifier that corresponds to the protein of 
                    your choice. These two identifiers must be separated by an underscore: e.g. 6VXX_A or AF-Q9BYF1-F1-model_v4_A or 
                    MGYP000740062793_A.</li>

                <li> There are <b>three presets</b> of redundant sets available to search into:
                    <ol>
                        <li> The viral dataset that is utilized in the method's manuscript that consists of 41674 structures 
                             &nbsp; (<a href={process.env.REACT_APP_BASE_URL + "/lists/viral_v1.csv"} download>Pub_Viral_PDB_Candidate_Set</a>).</li>
                        <li> A large human PDB dataset that includes 161629 structures and was retrieved by RSCB PDB
                             &nbsp; (<a href={process.env.REACT_APP_BASE_URL + "/lists/human_v1.csv"} download>Human_PDB_Candidate_Set_v1</a>).</li>
                        <li> The version 4 of the predicted human proteome dataset by AlphaFold DB (Human_AF4_Candidate_Set)</li>
                    </ol>
                </li>

                <li> Alternatively, you can select your own list of proteins that can be identifiers from the same sources that accepted
                     for declaring the reference protein. You can have a <b>mixed list</b> of identifiers originating from all three accepted 
                     sources (RCSB PDB, AlphaFold DB, ESM Atlas for MGnify). If you select a preset list, your custom list of structure 
                     IDs will be <b>ignored</b>.
                </li>

                <li> You are able to compare protein structures in <b>three levels</b>:
                    <ol>
                        <li><b>Whole structures</b>: The scanning will take into account all available information in each structure data file.</li>
                        <li><b>Domain</b>: All domains of the reference protein will be compared with all domains that are included in the proteins 
                            of the search space.</li>
                        <li><b>Segment</b>: A segment of the peptidic chain that refers to an area of your interest such as a binding site. The set of 
                            candidates is determined with an initial step of alignments by the level of your choice: primary protein structure, 
                            secondary protein structure, hydrophobicity or a representation that combines all for strictier alignment (mixed). 
                            The metrics are computed on the aligned segments (if there are any). Provide the range of the residue positions 
                            delimited by '-', e.g. 100-234. Due to limited resources, preset lists are not available for this mode.</li>
                    </ol>
                    Please take into account the <b>coverage</b> provided by available data. For example, a segment of your interest might not be present
                    in a particular PDB file or essential domain information is missing (referring to segment/domain comparison modes). In such cases, 
                    the submitted request will not be fullfilled. However, you could <Link to="/contact">let us know</Link> about it and we might come 
                    up with an alternative solution together.
                </li>
                <li> Machaon's <b>meta-analysis</b> extends the comparisons on the finalist proteins with established metrics and methodologies.  
                    The results are enriched with metadata and connected with genomic information.</li>

                <li> An additional step of meta-analysis is based on Gene Ontology information. You can provide a stem of a GO term 
                    in order to trace relevant proteins in the top 100 results. The stem is searched in all 3 GO term types (biological 
                    process, cellular component, molecular function). These relationships are also localized by protein 2D alignments 
                    that correlate parts of the reference structure with GO terms.</li>
                <li>
                    You will be able to submit your request once all the vields contain acceptable input as defined by the form's input 
                    filters. Red exclamation marks on the right of the fields designate an abnormal input. Please notice the short explanations
                    in gray for each input field. The submission button will be enabled when all the inputs for a legitimate request have been 
                    inserted.
                </li>

                <li> After a successful submission, a link will be generated that you need to <b>store</b> for accessing the result in a later 
                    time. Your request will be queued and it will be served depending on the total workload of the MachaonWeb network 
                    and your options. For example, the comparisons with a preset list that target a large dataset might take hours or 
                    enabling meta-analysis will increase the waiting time, especially in the early stages of MachaonWeb's operation that 
                    builds its cache from user queries. For now, if you miss the request link, you can make a new request after a while 
                    and access the results in a short time without waiting as the results are temporarily cached. In the next development iteration, 
                    we will provide a more user-friendly way for this issue.
                </li>

                <li>You will be able to access the results when the analysis you requested has been completed, by visiting the link that 
                    you kept during the request's submission. The lists of the chosen candidates are available in the page for a quick review via 
                    printer-friendly HTML-formatted files. All the produced outputs of the method are provided in a compressed file that 
                    includes files from the various stages of the analysis in <b>Excel-friendly or/and development-friendly format</b>. There are also <b>
                    data visualizations</b> of high resolution based on the proteins in the results wherever applicable (for requests with metanalysis 
                    enabled and large enough candidate lists). These are the areas covered by the plots:
                    <ul>
                        <li>Clustering (UMAP)</li>
                        <li>Metrics on the primary/secondary protein structure and genomic sequence (CDS, 3' UTR, 5' UTR) alignments</li>
                        <li>TM-Score (3D protein structure similarity)</li>
                        <li>Tanimoto Index (chemical similarity)</li>
                        <li>Similarity of Gene Ontology terms</li>
                        <li>Taxonomy trees</li>
                        <li>Word clouds based on organism names (or gene names for Human proteins)</li>
                        <li>Coverage of different fold types in protein secondary structure</li>
                        <li>Aggregated global alignments on protein secondary structure</li>    
                    </ul> For more
                    information on the outputs, please consult the <Link to="/citation">manuscript</Link> and the extended documentation in  <a 
                    href="https://github.com/anastasiadoulab/machaonweb">Github</a>.
                </li>

                <li>It is a good <b>security practice</b> to upload the downloaded compressed file of Machaon's outputs to <a href="https://www.virustotal.com/gui/home/upload">
                    VirusTotal</a> and compare the file's computed hash (displayed in the site's Details section after the upload) with the 
                    hash that is displayed in the Result page (SHA-256 hash) of MachaonWeb. Alternatively, you could try computing the hash offline
                    with console-based tools like <a href="https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/certutil">certutil
                    </a> in Windows or <a href="https://help.ubuntu.com/community/HowToSHA256SUM">sha256sum</a> in Unix-based systems. An exact match 
                    between the two hashes ensures that you have a valid copy of the results in our system.
                </li>

                <li> We suggest you to review the <Link to="/about">About</Link>, <Link to="/results">Sample results</Link> and <Link 
                 to="/policies">Policies</Link> sections to become more familiar with the service before you start using it.</li>
            </ul>
            Thank you for using Machaon! Feel free to <Link to="/contact">contact us</Link> for any issue.
        </Container>
    )    
}

export default Instructions;
