import React, { useEffect, useState } from 'react'; 
import { Container, ListGroup } from 'react-bootstrap';
import axios from 'axios';
import { useParams } from "react-router-dom";

function QuickViewLink({hash, filename, label}){
    return(
        <a target="_blank" rel="noopener noreferrer" href={process.env.REACT_APP_BASE_URL + "/output/"+ hash + "/" + filename }>{label}</a>
    )
}
// Display the html files for the quick view of the results
function QuickViewMultiLink({hash, filenames, label, nameSplitOffset}){
    const links = filenames.map( (filename) => {
        const name_parts = filename.split("_");
        let label_name = name_parts[name_parts.length - nameSplitOffset].split("-");
        label_name = label_name.slice(0, label_name.length -2);
        label_name = label_name.join(" ").replace("_", " ").replace("-", " ");
        return <li><QuickViewLink hash={hash} filename={filename} label={label_name}/></li>
    });
    return( 
        <span>{label}: <ul>{links}</ul></span>
    )
}

const Result = () => {

    const [request, setRequest] = useState({ reference: null, customList: '', listName: '', goTerm: '', 
                                             meta: false, comparisonMode: 0, segmentStart: -1, segmentEnd: -1, 
                                             secureHash: '', alignmentLevel: -1, statusCode: 0, creationDate: '-'});
    const [files, setFiles] = useState({ cluster: [], top: [], topHuman: [], goTerm: [] });
    const mode = ['whole', 'domain', 'segment'];
    const alignment_levels =  ['primary', 'secondary', 'hydrophobicity', 'mixed'];
    const { hash, reqid } = useParams(); 

    // Mapping between error codes and their corresponding messages
    const statusMessages = {'0' : 'This request is being processed and is not available yet. Please visit again later.',
                            '3' : '[Request failed] The reference did not include a chain ID as suffix.',
                            '4' : '[Request failed] Malformed values in request.',
                            '5' : '[Request failed] Erroneous value for preset list was given.',
                            '6' : '[Request failed] There was an unknown error. Please try again by checking your input or the status of the service. If the problem persists, please contact us',
                            '7' : '[Request failed] Reference structure was not able to be retrieved. ' +
                                  'Please review the reference structure id and try again.',
                            '8' : '[Request failed] No structure was able to be retrieved. '+
                                  'Please review your inputs and try again',
                            '9' : '[Request failed] Reference structure was not able to be retrieved. ' +
                                  'Please review the reference structure id and try again.',
                            '-1' : '[Request failed] There was an unknown error. Please try again by checking your input or the status of the service. If the problem persists, please contact us.'};

    // Request the server for the results of a request
    const fetchResult = async () => {
        await axios.get(process.env.REACT_APP_BASE_URL + '/resultdata/' + hash + '/'+ reqid)
        .then(function (response) {
            console.log(response);
            if (response.status === 200 && response.data.request.id > 0) { 

                const datetime = new Date(response.data.request.creation_date);
                const formattedDate = datetime.toLocaleDateString('en-US', { 
                  year: 'numeric',  month: 'long',  day: 'numeric', 
                  hour: 'numeric',   minute: 'numeric',  second: 'numeric', 
                  hour12: true 
                });

                setRequest(previous => ({ ...previous, 
                           reference: response.data.request.reference,
                           customList: response.data.request.custom_list.length > 0 
                                       ? response.data.request.custom_list.replace(new RegExp(',', 'g'), ', ') : '',
                           listName: response.data.request.list_name !== null ? response.data.request.list_name : '',
                           goTerm: response.data.request.go_term !== '' ? response.data.request.go_term  : '',
                           meta: response.data.request.meta,
                           comparisonMode: response.data.request.comparison_mode,
                           segmentStart: response.data.request.segment_start,
                           segmentEnd: response.data.request.segment_end,
                           secureHash: response.data.request.secure_hash,
                           alignmentLevel: response.data.request.alignment_level,
                           statusCode: response.data.request.status_code,
                           creationDate: formattedDate
                 }));
                 setFiles(previous => ({ ...previous, ...response.data.files }));
            }
            else throw "Bad request.";
             
        })
        .catch(function (error) {
            setRequest(previous => ({ ...previous, reference: '' }));
            console.log(error);
        })
    }  
    
    useEffect(() => {
        fetchResult();
    },[]);
    

    return ( 
        <Container>
            <h1>Result</h1>
            {(request.reference ?? 'null') === '' && <div className="no-result-message">
                This request have not been processed yet or it does not exist. Please review your link or visit again a while later.
            </div>}
            {(request.secureHash === '' &&  (request.reference ?? '') !== '')  &&<div className="no-result-message">
                {request.statusCode in statusMessages ? statusMessages[request.statusCode] : statusMessages['-1']} 
            </div>}
            { (request.secureHash ?? '') !== '' && <div className="result-container">
                <h3>Request details</h3>
                <ListGroup>
                    <ListGroup.Item><span className="list-item-label">Reference structure: </span>{request.reference}</ListGroup.Item>
                    <ListGroup.Item><span className="list-item-label">Comparison mode: </span>
                        {mode[request.comparisonMode] } 
                        {request.comparisonMode === 2 &&<span> | <b>Alignment level:</b> {alignment_levels[request.alignmentLevel] }</span>}
                    </ListGroup.Item>
                    <ListGroup.Item><span className="list-item-label">Candidates: </span>{ request.customList !== '' ? request.customList : request.listName }</ListGroup.Item>
                    { (request.segmentStart > 0 && request.segmentEnd > 0) &&
                    <ListGroup.Item><span className="list-item-label">Residue range: </span>{  request.segmentStart + '-' + request.segmentEnd}</ListGroup.Item>
                    }
                    <ListGroup.Item><span className="list-item-label">Meta-analysis enabled: </span>{ request.meta === true ? 'Yes' : 'No'}</ListGroup.Item>
                    { (request.meta === true && request.goTerm !== '') &&
                    <ListGroup.Item><span className="list-item-label">GO term: </span>{ request.goTerm }</ListGroup.Item>
                    }
                    <ListGroup.Item><span className="list-item-label">Submitted at: </span>{request.creationDate + ' (UTC)'}</ListGroup.Item>
                </ListGroup>  
                <br/> 
                <br/> 
                <h3>Quick view</h3>
                <span className='italics-note'>(Links open in a new tab or window.)</span>
                <ul className="view-options">
                    <li style={{display: files.top.length > 0 ? 'list-item' : 'none'}}>
                        {files.top.length === 1 && <QuickViewLink hash={hash} filename={files.top[0]} label={"Top 100"}/>}  
                        {files.top.length > 1 && <QuickViewMultiLink hash={hash} filenames={files.top} label={"Top 100"} nameSplitOffset={3}/>}  
                    </li>
                    <li style={{display: files.topHuman.length > 0 ? 'list-item' : 'none'}}>
                        {files.topHuman.length === 1 && <QuickViewLink hash={hash} filename={files.topHuman[0]} label={"Top 100 (Human only)"}/>} 
                        {files.topHuman.length > 1 && <QuickViewMultiLink hash={hash} filenames={files.topHuman} label={"Top 100 (Human only)"} nameSplitOffset={2}/>} 
                    </li>
                    <li>
                        {files.cluster.length === 1 && <QuickViewLink hash={hash} filename={files.cluster[0]} label={"Final cluster"}/>}  
                        {files.cluster.length > 1 && <QuickViewMultiLink hash={hash} filenames={files.cluster} label={"Final cluster"} nameSplitOffset={2}/>}   
                    </li>
                    <li style={{display: files.goTerm.length > 0 ? 'list-item' : 'none'}}>
                        {files.goTerm.length === 1 && <QuickViewLink hash={hash} filename={files.goTerm[0]} label={"Proteins associated with selected GO term"}/>}  
                        {files.goTerm.length > 1 && <QuickViewMultiLink hash={hash} filenames={files.goTerm} label={"Proteins associated with selected GO term"} nameSplitOffset={2}/>} 
                    </li>
                </ul>
                <br/> 
                <h3>Detailed Analysis</h3>
                <a href={process.env.REACT_APP_BASE_URL + "/output/"+ hash + "/" + hash + ".zip"} >Download all outputs</a><br/>
                <div className="secure-hash">SHA-256 Hash: {request.secureHash}</div>
            </div> }
        </Container>
    )    
}

export default Result;
