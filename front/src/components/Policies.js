import { Container } from 'react-bootstrap';
import { Link } from "react-router-dom";

// Text for 'Policies' section
const Policies = () => { 
    
    return ( 
        <Container>
            <h1>Policies</h1> 
            
            <h3>Privacy</h3>
            We do not retain any personal information for the usage of this service and the results cannot be traced back to 
            those who requested them. However, the results are public and can be viewed by anyone who is aware of the corresponding link.
            The identifier in the link is a non-encrypting hash of the request's details for caching purposes. Please be noted that 
            Google Analytics are employed in this platform for usage statistics and resource planning.
            <br/><br/>
            <h3>Workload</h3>
            In order to protect our resources and allow as many people as possible to access Machaon, we throttle the total rate of 
            requests in a variable time interval. This means that the server cannot accept several requests at the same time. The 
            submission process is guarded by input validation and Google reCaptcha v3. All users are able to see the load of the 
            network and act accordingly and responsibly. If you need more from the method, please <Link to="/contact">contact us</Link> to 
            help you.
            <br/><br/>
            <h3>Data</h3>
            For the time being, the results are maintained to the server based on their access rate and can be shared with anyone. This 
            might change anytime to ensure the viability of this project. We do not guarantee the long-term availability of the data, 
            therefore offline data retention is recommended. Also, you should check the file hashes of the compressed files that you 
            download (there are some suggestions in the <Link to="/instructions">instructions</Link>) and interact with them in a 
            protected and updated system.
        </Container>
    )    
}

export default Policies;
