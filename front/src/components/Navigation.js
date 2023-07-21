import React, { useEffect, useState } from 'react';
import { Badge, Nav, Navbar, Container } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';

import axios from 'axios';

// The navigation menu of the SPA
const Navigation = () => {

    const [systemInfo, setSystemInfo] = useState({listItems: [], nodes: 0, jobs: 0, queued: 0});
 
    // Retrieve the status of the MachaonWeb network
    const fetchSystemData = async () => {
        await axios.get(process.env.REACT_APP_BASE_URL + '/info')
        .then(function (response) {
            console.log(response);
            if (response.status === 200){ 
                setSystemInfo(previous => ({ ...previous, listItems: response.data.candidate_lists, 
                                              nodes: response.data.nodes, jobs: response.data.jobs,
                                              queued: response.data.queued}));
            }
        })
        .catch(function (error) {
            console.log(error);
        })
    }  
    
    useEffect(() => {
        fetchSystemData();
    }, [])

    return (
       <Navbar bg="dark" variant="dark">
        <Container>
          <Nav className="me-auto">
            <LinkContainer to="/">
                <Nav.Link>Home</Nav.Link>
            </LinkContainer>
            <LinkContainer to="/instructions">
                <Nav.Link>Instructions</Nav.Link>
            </LinkContainer>
            <LinkContainer to="/about">
                <Nav.Link>About</Nav.Link>
            </LinkContainer>
            <LinkContainer to="/results">
                <Nav.Link>Sample results</Nav.Link>
            </LinkContainer>
            <LinkContainer to="/policies">
                <Nav.Link>Policies</Nav.Link>
            </LinkContainer> 
            <LinkContainer to="/contact">
                <Nav.Link>Contact</Nav.Link>
            </LinkContainer>
            <LinkContainer to="/citation">
                <Nav.Link>Citation</Nav.Link>
            </LinkContainer>
            <div className="nav-info-text">
            <Navbar.Text className='sys-info ms-auto'>
                Nodes: <Badge className="nav-badge">{systemInfo.nodes}</Badge>
                 <span className="nav-sep">|</span> Jobs Running: 
                 <Badge className="nav-badge job_count">{systemInfo.jobs}</Badge> 
                 <span className="nav-sep">|</span> Jobs Queued: 
                 <Badge className="nav-badge job_count">{systemInfo.queued}</Badge>
            </Navbar.Text> 
          </div>
          </Nav>
          <div className="nav-arrow" aria-hidden="true">&gt;</div> 
        </Container>
      </Navbar>
    )
       
}

export default Navigation;
