import { Col, Container, Row } from 'react-bootstrap';

const hypatiaLogo = require('../assets/hypatia-logo-v1.png');
const uoaLogo = require('../assets/uoa_logo_eng.png');
const brfaaLogo = require('../assets/logo-brfaa.jpg');

// Responsive footer
const Footer = () => {
      
    return ( 
        <footer className="site-footer">
            <Container fluid>
                <Row>
                    <Col className='left-footer'>
                        <img alt='University of Athens (UoA) logo' className='uoa-logo' src={String(uoaLogo)} />
                        <img alt=' Biomedical Research Foundation of the Academy of Athens (BRFAA) logo' className='brfaa-logo' src={String(brfaaLogo)} />
                    </Col>
                    <Col className='middle-footer'><span>© 2023</span></Col>
                    <Col className='right-footer'>
                        <span className="hypatia-label">Powered by</span>  
                        <img alt='Hypatia cluster logo' className='hypatia-logo' src={String(hypatiaLogo)} />
                    </Col>
                </Row>
            </Container>
        </footer>
    )    
}

export default Footer;
