import React, { useEffect, useCallback, useState } from 'react'; 
import { Col, Modal, Form, Button, Container, } from 'react-bootstrap';
import { Link } from "react-router-dom";
import axios from 'axios';
import { useGoogleReCaptcha } from 'react-google-recaptcha-v3';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import { extractStructureID, extractCompositeID} from "../utils.js"; 


const RequestForm = () => {
    
    const [request, setRequest] = useState({ reference: '', customList: '', candidateList: -1, goTerm: '', 
                                             meta: false, comparisonMode: 0, segmentStart: -1, segmentEnd: -1,
                                            token: '', alignmentLevel: 3 });
    const { executeRecaptcha } = useGoogleReCaptcha();
    const [candidateLists, setLists] = useState([]);
    const [modalDetails, setModalDetails] = useState({ show: false, text: '', requestLink: '' });
    const [copying, setCopying] = useState({ buttonText: 'Copy', text: '' });
    const [submission, setSubmission] = useState({ buttonText: 'Submit', formEnabled: true });
    const [validation, setValidation] = useState({ reference: null, customList: null, residueRange: null });
    
    // Mapping between error codes and their corresponding messages
    const statusMessages = { '0' : 'Your request was queued. Please keep the following link and visit after a while:',
                            '-1' : 'There is a problem with the provided reference id.',
                            '-2' : 'Another request was just queued some moments ago. We throttle the total'+
                                   ' request rate of all users due to limited resources. Please try a while later.',
                            '-3' : 'The selected candidate list identifier is not present in the system.',
                            '-4' : 'Please re-check the custom list of candidates.',
                            '-5' : 'Please re-check the custom list of candidates.',
                            '-6' : 'Please re-check the custom list of candidates.',
                            '-7' : 'Please re-check the choice of residue range.',
                            '-8' : 'Your request was not validated by reCaptcha. Please try again.',
                            '-9' : 'Invalid option for comparison mode.',
                            '-10' : 'Invalid option for segment alignment level.',
                            '1' : 'Unknown error. Please check your input or the status of the service.'};

    // Change button text on copy
    const handleCopy =  () => {
        setCopying(previous => ({ ...previous, buttonText: 'Copied'}));
    };

    // Filter user input for Gene Ontology terms
    const extractGOterm = (input_text) => {
        const regex = /[A-Z|0-9|a-z|\s]+/g; 
        let matched = input_text.match(regex);
        setRequest(previous => ({ ...previous, goTerm: matched  !== null ? matched : ''})); 
    }

    // Filter user input for reference structure
    const handleTextChange = (event) => { 
        const referenceId = extractCompositeID(event.target.value.trim());
        if(referenceId.length > 0)
        {
            setRequest(previous => ({ ...previous, reference: referenceId}));
        }   
        setValidation(previous => ({ ...previous, reference: referenceId.length > 0 }));
    }

    // Handle the dismission of a modal
    const handleModalClose = () => {
        setCopying(previous => ({ ...previous, buttonText: 'Copy'}));
        setModalDetails(previous => ({ ...previous, show: false}));
        setSubmission(previous => ({ ...previous, buttonText: 'Submit', formEnabled: true}));
    }

    // Handle the opening of a modal
    const handleModalShow = (content, link) => setModalDetails(previous => ({ ...previous, show: true, text: content, requestLink: link}));
    
    // Event handle for GO term selection
    const setGoTerm = (event) => { 
        extractGOterm(event.target.value.trim());
    }

    // Event handle for meta-analysis selection
    const setChecked = (event) => { 
        const checked = event.target.checked; 
        setRequest(previous => ({ ...previous, meta: checked}));
        if(checked === false){ 
            setRequest(previous => ({ ...previous, goTerm: ''}));
        }
    }
    
    // Selecting a preset list
    const handleListChange = (event) => {
        setRequest(previous => ({ ...previous, candidateList: parseInt(event.target.value), customList: ''}));
    }

    // Handle the selection of alignment level for the target of feature extraction (segment scanning only)
    const handleAlignmentLevelSelection = (event) => {
        setRequest(previous => ({ ...previous, alignmentLevel: parseInt(event.target.value) }));
    }

    // Handle the selection of residues by the user
    const handleResidueChoice = (event) => {
        let residuesChoice = event.target.value.split('-');
        let segment_start = -1;
        let segment_end = -1;
        let invalid = true;
        if(residuesChoice.length === 2) {
            segment_start = parseInt(residuesChoice[0]);
            segment_end = parseInt(residuesChoice[1]);
            if(isNaN(segment_start) === false && isNaN(segment_end) === false) {
                // Setting some size limits on the selection
                if(segment_start < segment_end && 
                    segment_end < 10000 && 
                    segment_start < 10000 && 
                    segment_end > 0 && 
                    segment_start > 0 &&
                    segment_end - segment_start < 600 &&
                    segment_end - segment_start > 2)
                {
                    setRequest(previous => ({ ...previous, segmentStart: segment_start, segmentEnd: segment_end}));
                    invalid = false; 
                }
            }
        }
        // Validate input
        if (invalid === true)
        {
            setRequest(previous => ({ ...previous, segmentStart: -1, segmentEnd: -1})); 
        }
        setValidation(previous => ({ ...previous, residueRange: !invalid }));
    }

    // Handle the selection of comparison level
    const handleComparisonSelection = (event) => {
        const mode = parseInt(event.target.value);
        let listSelection = request.candidateList;
        if(mode === 2)
        {
            listSelection = -1;
        }
        setRequest(previous => ({ ...previous, candidateList: listSelection, comparisonMode: mode, 
                   segmentStart: -1, segmentEnd: -1}));
    }
    
    // Handle the user input about specific candidate structure IDs
    const handleTextareaChange = (event) => {
        let customIds = event.target.value.trim();
        let currentId = "";
        let extractedIds = [];

        if (customIds.length > 0)
        {
            customIds = customIds.split(",");

            for(const structureId of customIds)
            {
                if (structureId.trim().length === 0){
                    continue;
                }
                currentId = extractStructureID(structureId.trim());
                if(currentId === "")
                    break;
                else
                    extractedIds.push(currentId);
            }
            if(currentId !== "")
            {
                setRequest(previous => ({ ...previous, candidateList: -1, customList: extractedIds.join(',')}));
            }
        }
        setValidation(previous => ({ ...previous, customList: customIds.length > 0 ? currentId !== "" : null  }));
    }
                     
    // Create an event handler so you can call the verification on button click event or form submit
    const handleReCaptchaVerify = useCallback(async () => {
        if (!executeRecaptcha) {
            console.log('Execute recaptcha not yet available');
            return true;
        }
        const captcha_token = await executeRecaptcha('submission');
        // Do whatever you want with the token
        setRequest(previous => ({ ...previous, token: captcha_token}));

    }, [executeRecaptcha]);

    // Handle the submission of user input as a request to MachaonWeb
    const submitRequest = (event) => {
       handleReCaptchaVerify(); 
       setSubmission(previous => ({ ...previous, buttonText: 'Submitting, please wait...', formEnabled: false}));
       axios.post(process.env.REACT_APP_BASE_URL + '/request', request)
      .then(function (response) {
        console.log(response);
        let status_code = '1';
        let requestLink = '';
        if(response.status === 200) {
            status_code = response.data.status_code.toString();
            if(status_code === "0") { 
                requestLink = process.env.REACT_APP_BASE_URL + '/result/' + response.data.hash.toString()+ "/" + response.data.request_id.toString(); 
            }
            if(!(status_code in statusMessages))
            {
                status_code = '1';
            }
            setCopying(previous => ({ ...previous, text: requestLink }));
        }
        handleModalShow(statusMessages[status_code], requestLink);
      })
      .catch(function (error) {
        console.log(error);
        handleModalShow(statusMessages['1'], '');
      })
    }

    // Retrieve and show the available preset lists for candidate structure sets 
    const fetchLists = async () => {
        await axios.get(process.env.REACT_APP_BASE_URL + '/lists')
        .then(function (response) {
            console.log(response);
            if (response.status === 200) { 
                setLists(response.data);
            }
        })
        .catch(function (error) {
            console.log(error);
        })
    }  
    
    useEffect(() => {
        fetchLists();
    }, [])
    
    return (
        <Container>
            <Modal show={ modalDetails.show } onHide={handleModalClose}>
                <Modal.Header closeButton> 
                </Modal.Header>
                <Modal.Body>{ modalDetails.text }
                    { modalDetails.requestLink !== '' && 
                    <div className='request-link-container'>
                        <div className='request-link'>
                            <a href={ modalDetails.requestLink } target="_blank" rel="noopener noreferrer" className='request-link-anchor'>
                                { modalDetails.requestLink }
                            </a> 
                            <CopyToClipboard text={copying.text} onCopy={handleCopy}>
                                <Button className='copy-button'>{copying.buttonText}</Button>
                            </CopyToClipboard> 
                        </div> 
                    </div> }
                </Modal.Body>
            </Modal>
            <Form>
                <Form.Group className='mb-3' controlId='form.Reference'>
                    <Form.Label>1. Choose a reference structure <span 
                     className="short-note">(Structure ID & Chain ID e.g. 6VXX_A or AF-Q9BYF1-F1-model_v4_A)</span>:
                    </Form.Label>
                    <Form.Control name='reference' onChange={handleTextChange} maxLength={40} type='text'/> 
                    {validation.reference === false  && <span className="error-mark" aria-hidden="true">!</span>}
                </Form.Group>
            
                <Form.Group as={Col} className='mb-3' controlId='form.CandidateList'>
                    <Form.Label>2. Choose a preset list of candidates <span 
                     className="short-note">(For whole/domain comparisons only)</span>:</Form.Label>
                    <Form.Control disabled={request.comparisonMode === 2} as='select' name='candidateList' 
                        value={request.candidateList} onChange={handleListChange}>
                        <option value='-1'>-</option>
                        {candidateLists.map(({id, title}) => (
                            <option key={id} value={id}>{title}</option>
                        ))}
                    </Form.Control>
                    <Form.Label>Or provide your own list of candidates <span 
                     className="short-note">(Structure IDs e.g. 3D0H, AF-P00533-F1-model_v4, MGYP000740062793. By selecting a preset list, the custom list is ignored.)</span>:
                </Form.Label>
                    <Form.Control name='customList' as='textarea' rows={3} maxLength={5000} 
                    onChange={handleTextareaChange} /> 
                    {validation.customList === false  && <span className="error-mark" aria-hidden="true">!</span>}
                </Form.Group>

                <Form.Group as={Col} className='mb-3' controlId='form.ComparisonMode'>
                    <Form.Label>3. Select the comparison mode: </Form.Label>
                    <Form.Control as='select' name='comparisonMode' value={request.comparisonMode} onChange={handleComparisonSelection}>
                        <option value='0'>Whole Structure</option>
                        <option value='1'>Domain</option>
                        <option value='2'>Segment</option>
                    </Form.Control>
                    {request.comparisonMode === 2 && <span className='mode-select'>
                        <div className="segment-mode">
                        <Form.Label>3a. Provide a range of residues:</Form.Label>
                        <Form.Control className='residue-choice' maxLength={10} name='residues' onChange={handleResidueChoice} type='text' placeholder='e.g. 10-100' />
                        {validation.residueRange === false  && <span className="error-mark" aria-hidden="true">!</span>}
                        <Form.Label>3b. Select the alignment level: </Form.Label>
                        <Form.Control as='select' className='alignment-choice' name='alignmentLevel' value={request.alignmentLevel} onChange={handleAlignmentLevelSelection}>
                            <option value='0'>Primary</option>
                            <option value='1'>Secondary</option>
                            <option value='2'>Hydrophobicity</option>
                            <option value='3'>Mixed</option>
                        </Form.Control>
                        </div>
                    </span>}
                </Form.Group>
                
                <Form.Group className='mb-3' controlId='form.Meta'>
                    <Form.Check name='meta' onChange={setChecked} type='checkbox' label='4. Perform meta-analysis (optional)' />
                    <Form.Label>Provide a term for Gene Ontology Analysis (it can be left empty):</Form.Label>
                    <Form.Control name='goTerm' type='text' maxLength={100} value={request.goTerm} onChange={setGoTerm} disabled={!request.meta} placeholder='e.g. angiogenesis' />
                </Form.Group> 
            </Form>
           <div className="submit-section">
                <Button className='btn-submit' variant='primary' onClick={submitRequest} 
                    disabled={(request.candidateList > 0 || validation.customList === true) && 
                                submission.formEnabled === true &&
                                validation.reference === true  &&
                                (request.comparisonMode  === 2 ? validation.residueRange === true : true) 
                                ? false : true }>{ submission.buttonText }</Button> 
                <div className="short-note"> (By submitting a request, you are confirming that you are aware of the current <Link to="/policies">policies</Link>.)</div>
             </div>
        </Container>
    )
}

export default RequestForm;
