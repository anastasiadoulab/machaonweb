import { Link } from "react-router-dom";
import { Container } from 'react-bootstrap';

// The displayed for unhandled requested routes
const NoMatch = () => {
      
    return (  
        <Container>
            <h2>Nothing to see here!</h2>
            <p> 
                <Link to="/">Go to the home page</Link>
            </p>
        </Container>
    )    
}

export default NoMatch;
