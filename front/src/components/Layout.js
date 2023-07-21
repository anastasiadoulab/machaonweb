import Navigation from "./Navigation";
import Footer from "./Footer";
import { Outlet, useLocation } from "react-router-dom";
import { Stack } from 'react-bootstrap';

// The layout of the SPA
const Layout = () => {

    const disableBackToTopPaths = ['/citation', '/contact'];
    const routerLocation = useLocation();

    return (
        <Stack direction='vertical' gap={3}>
            <a id="#top" className="top-anchor" aria-hidden="true"></a>
            <Navigation />
            <div className='main-container'>
                <Outlet />
                { !disableBackToTopPaths.includes(routerLocation.pathname) &&
                <div className="back-to-top"><a href="#top">Back to top</a></div>
                }
            </div> 
            <Footer />
        </Stack>
    )    
}

export default Layout;
